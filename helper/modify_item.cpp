#include <QCoreApplication>
#include <Akonadi/Item>
#include <Akonadi/ItemModifyJob>
#include <iostream>

int main(int argc, char *argv[]) {
    QCoreApplication app(argc, argv);

    if (argc < 3) {
        std::cerr << "Usage: akonadi_helper <item_id> <new_remote_id>\n";
        return 1;
    }

    qint64 itemId = QString(argv[1]).toLongLong();
    QString newRemoteId(argv[2]);

    // Update item with Akonadi API
    // Update remote ID (e.g., after moving file in maildir)
    Akonadi::Item item(itemId);
    item.setRemoteId(newRemoteId);

    Akonadi::ItemModifyJob *modifyJob = new Akonadi::ItemModifyJob(item);
    modifyJob->setIgnorePayload(true);  // Don't update payload, just metadata


    return app.exec();
}
