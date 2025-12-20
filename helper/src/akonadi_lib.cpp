#include "akonadi_lib.h"
#include <QCoreApplication>
#include <Akonadi/Item>
#include <Akonadi/ItemModifyJob>
#include <Akonadi/ItemDeleteJob>
#include <QEventLoop>
#include <QString>
#include <string>

static std::string last_error;

static QCoreApplication *qt_app = nullptr;
static int argc = 1;
static char app_name[] = "akonadi_helper";
static char *argv[] = {app_name, nullptr};

// Initialize Qt application (call once)
static void ensure_qt_app()
{
    if (!qt_app)
    {
        qt_app = new QCoreApplication(argc, argv);
    }
}

extern "C" int modify_pimitem(long long item_id, const char *remote_id)
{
    ensure_qt_app();

    try
    {
        Akonadi::Item item(item_id);
        item.setRemoteId(QString::fromUtf8(remote_id));

        QEventLoop loop;
        auto *job = new Akonadi::ItemModifyJob(item);
        job->setIgnorePayload(true);

        QObject::connect(job, &Akonadi::ItemModifyJob::result, [&](KJob *j)
                         {
            if (j->error()) {
                last_error = j->errorString().toStdString();
                loop.exit(1);
            } else {
                loop.exit(0);
            } });

        return loop.exec();
    }
    catch (const std::exception &e)
    {
        last_error = e.what();
        return -1;
    }
}

extern "C" int delete_pimitem(long long item_id)
{
    ensure_qt_app();

    try
    {
        Akonadi::Item item(item_id);

        QEventLoop loop;
        auto *job = new Akonadi::ItemDeleteJob(item);

        QObject::connect(job, &Akonadi::ItemDeleteJob::result, [&](KJob *j)
                         {
            if (j->error()) {
                last_error = j->errorString().toStdString();
                loop.exit(1);
            } else {
                loop.exit(0);
            } });

        return loop.exec();
    }
    catch (const std::exception &e)
    {
        last_error = e.what();
        return -1;
    }
}

extern "C" const char *get_last_error()
{
    return last_error.c_str();
}

extern "C" void cleanup_qt_app()
{
    if (qt_app)
    {
        delete qt_app;
        qt_app = nullptr;
    }
}
