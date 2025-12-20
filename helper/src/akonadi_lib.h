#ifndef AKONADI_LIB_H
#define AKONADI_LIB_H

#ifdef __cplusplus
extern "C"
{
#endif

    // Return 0 on success, non-zero on error
    int modify_pimitem(long long item_id, const char *remote_id);
    int delete_pimitem(long long item_id);

    // Get last error message (returns pointer to internal buffer)
    const char *get_last_error();

    // Cleanup Qt application (call before program exit)
    void cleanup_qt_app();

#ifdef __cplusplus
}
#endif

#endif // AKONADI_LIB_H
