#include <QCoreApplication>
#include <Akonadi/Item>
#include <Akonadi/ItemDeleteJob>
#include <iostream>

int main(int argc, char *argv[]) {
    QCoreApplication app(argc, argv);

    if (argc < 2) {
        std::cerr << "Usage: delete_pimitem <item_id>\n";
        std::cerr << "Deletes a pimitem from Akonadi database without touching payload.\n";
        return 1;
    }

    qint64 itemId = QString(argv[1]).toLongLong();

    if (itemId <= 0) {
        std::cerr << "Error: Invalid item ID: " << itemId << "\n";
        return 1;
    }

    std::cout << "Deleting pimitem with ID: " << itemId << "\n";

    // Delete item using Akonadi API
    Akonadi::Item item(itemId);
    Akonadi::ItemDeleteJob *deleteJob = new Akonadi::ItemDeleteJob(item);

    QObject::connect(deleteJob, &Akonadi::ItemDeleteJob::result, [&app](KJob *job) {
        if (job->error()) {
            std::cerr << "Error deleting item: " << job->errorString().toStdString() << "\n";
            app.exit(1);
        } else {
            std::cout << "Item deleted successfully.\n";
            app.exit(0);
        }
    });

    return app.exec();
}
