//
//  VcxCallbacks.m
//  libindy
//

#include "vcx.h"
#import "VcxCallbacks.h"
#import "NSError+VcxError.h"
#import "VcxTypes.h"
#import "VcxErrors.h"

static NSString *commandCallbackKey = @"commandCallback";


@interface VcxCallbacks ()

@property(strong, readwrite) NSMutableDictionary *commandCompletions;
@property int32_t commandHandleCounter;
@property(strong, readwrite) NSRecursiveLock *globalLock;

@end

@implementation VcxCallbacks

+ (VcxCallbacks *)sharedInstance {
    static VcxCallbacks *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [VcxCallbacks new];
    });

    return instance;
}

- (VcxCallbacks *)init {
    self = [super init];
    if (self) {
        self.commandHandleCounter = 0;
        self.commandCompletions = [[NSMutableDictionary alloc] init];
        self.globalLock = [NSRecursiveLock new];
    }
    return self;
}

// MARK: - Create command handle and store callback

- (vcx_command_handle_t)createCommandHandleFor:(id)callback {
    NSNumber *handle = nil;

    @synchronized (self.globalLock) {
        handle = [NSNumber numberWithInt:self.commandHandleCounter];
        self.commandHandleCounter++;

        NSMutableDictionary *dict = [NSMutableDictionary new];
        dict[commandCallbackKey] = [callback copy];

        self.commandCompletions[handle] = dict;
    }
    return (vcx_command_handle_t) [handle integerValue];
}

- (void)deleteCommandHandleFor:(vcx_command_handle_t)handle {
    NSNumber *key = [NSNumber numberWithInt:handle];
    @synchronized (self.globalLock) {
        if ([self.commandCompletions objectForKey:key]) {
            [self.commandCompletions removeObjectForKey:key];
        }
    }
}

- (id)commandCompletionFor:(vcx_command_handle_t)handle {
    NSNumber *key = [NSNumber numberWithInt:handle];
    id val = nil;
    @synchronized (self.globalLock) {
        NSMutableDictionary *dict = (NSMutableDictionary *) [self.commandCompletions objectForKey:key];
        val = [dict objectForKey:@"commandCallback"];
    }
    return val;
}

- (void)complete:(void (^)(NSError *))completion
       forHandle:(vcx_command_handle_t)handle
         ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret]);
        });
    }
}

- (void)completeBool:(void (^)(NSError *, BOOL))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], false);
        });
    }
}

- (void)completeStr:(void (^)(NSError *, NSString *))completion
          forHandle:(vcx_command_handle_t)handle
            ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], nil);
        });
    }
}

- (void)complete2Str:(void (^)(NSError *, NSString *, NSString *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], nil, nil);
        });
    }
}

- (void)completeData:(void (^)(NSError *, NSData *))completion
           forHandle:(vcx_command_handle_t)handle
             ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], nil);
        });
    }
}

- (void)complete2Data:(void (^)(NSError *, NSData *, NSData *))completion
            forHandle:(vcx_command_handle_t)handle
              ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], nil, nil);
        });
    }
}

- (void)completeStringAndData:(void (^)(NSError *, NSString *, NSData *))completion
                    forHandle:(vcx_command_handle_t)handle
                      ifError:(vcx_error_t)ret {
    if (ret != Success) {
        [self deleteCommandHandleFor:handle];
        dispatch_async(dispatch_get_main_queue(), ^{
            completion([NSError errorFromVcxError:ret], nil, nil);
        });
    }
}

@end

// MARK: - static indy C-callbacks

void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *) = (void (^)(NSError *)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error);
        });
    }
}

void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        vcx_command_handle_t pool_handle) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, VcxHandle) = (void (^)(NSError *, VcxHandle)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (VcxHandle) pool_handle);
        });
    }
}

void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        int32_t handle) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSNumber *) = (void (^)(NSError *, NSNumber *arg1)) block;
    NSNumber *sarg1 = @(handle);

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (NSNumber *) sarg1);
        });
    }
}

void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *) = (void (^)(NSError *, NSString *arg1)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1);
        });
    }
}

void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        unsigned int arg1) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, BOOL) = (void (^)(NSError *, BOOL arg1)) block;

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (BOOL) arg1);
        });
    }
}

void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2) = (void (^)(NSError *, NSString *arg1, NSString *arg2)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = nil;
    if (arg1) {
        sarg2 = [NSString stringWithUTF8String:arg2];
    }

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = nil;
    if (arg2) {
        sarg2 = [NSString stringWithUTF8String:arg2];
    }
    NSString *sarg3 = nil;
    if (arg3) {
        sarg3 = [NSString stringWithUTF8String:arg3];
    }

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const char *const arg2,
        const char *const arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSString *arg3)) block;

    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSString *sarg3 = [NSString stringWithUTF8String:arg3];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2, sarg3);
        });
    }
}

/// Arguments arg1 and arg2 will be converted to nsdata
void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const uint8_t *const arg1,
        uint32_t arg2) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSData *arg) = (void (^)(NSError *, NSData *arg)) block;

    NSData *sarg = [NSData dataWithBytes:arg1 length:arg2];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg);
        });
    }
}

void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *const arg1,
        const uint8_t *const arg2,
        uint32_t arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *, NSData *) = (void (^)(NSError *, NSString *, NSData *)) block;

    NSString *sarg1 = nil;
    if (arg1) {
        sarg1 = [NSString stringWithUTF8String:arg1];
    }
    NSData *sarg2 = [NSData dataWithBytes:arg2 length:arg3];

    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, sarg1, sarg2);
        });
    }
}

void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
        vcx_error_t err,
        const char *arg1,
        const char *arg2,
        unsigned long long arg3) {
    id block = [[VcxCallbacks sharedInstance] commandCompletionFor:xcommand_handle];
    [[VcxCallbacks sharedInstance] deleteCommandHandleFor:xcommand_handle];

    void (^completion)(NSError *, NSString *, NSString *, NSNumber *) = (void (^)(NSError *, NSString *arg1, NSString *arg2, NSNumber *arg3)) block;
    NSString *sarg1 = [NSString stringWithUTF8String:arg1];
    NSString *sarg2 = [NSString stringWithUTF8String:arg2];
    NSNumber *sarg3 = [NSNumber numberWithInt:arg3];


    if (completion) {
        dispatch_async(dispatch_get_main_queue(), ^{
            NSError *error = [NSError errorFromVcxError:err];
            completion(error, (NSString *) sarg1, (NSString *) sarg2, (NSNumber *) sarg3);
        });
    }
}
