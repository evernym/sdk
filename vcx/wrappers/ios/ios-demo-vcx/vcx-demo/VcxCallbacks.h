//
//  VcxCallbacks.h
//


#import <Foundation/Foundation.h>
#import "vcx.h"

extern void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err);

extern void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        vcx_command_handle_t pool_handle);

extern void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1);

extern void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        unsigned int arg1);

extern void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2);

extern void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2);

extern void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const uint8_t *const arg1,
        uint32_t arg2);

extern void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3);

extern void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const uint8_t *const arg2,
        uint32_t arg3);

extern void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        int32_t handle);

extern void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3);

void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *arg1,
        const char *arg2,
        unsigned long long arg3);

@interface VcxCallbacks : NSObject

// MARK: - Store callback and create command handle
- (vcx_command_handle_t)createCommandHandleFor:(id)callback;

- (id)commandCompletionFor:(vcx_command_handle_t)handle;

- (void)deleteCommandHandleFor:(vcx_command_handle_t)handle;

- (void)complete:(void (^)(NSError *))completion
       forHandle:(vcx_command_handle_t)handle
         ifError:(vcx_error_t)ret;

- (void)completeBool:(void (^)(NSError *, BOOL))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;

- (void)completeStr:(void (^)(NSError *, NSString *))completion
          forHandle:(vcx_command_handle_t)handle
            ifError:(vcx_error_t)ret;

- (void)complete2Str:(void (^)(NSError *, NSString *, NSString *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;

- (void)completeData:(void (^)(NSError *, NSData *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret;


- (void)completeStringAndData:(void (^)(NSError *, NSString *, NSData *))completion
                    forHandle:(vcx_command_handle_t)handle
                      ifError:(vcx_error_t)ret;

+ (VcxCallbacks *)sharedInstance;

@end
