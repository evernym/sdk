//
//  init.m
//  vcx
//
//  Created by GuestUser on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "ConnectMeVcx.h"
#import "utils/NSError+VcxError.h"
#import "utils/VcxCallbacks.h"
#import "vcx.h"
#include "vcx.h"

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




@implementation ConnectMeVcx

- (void)createOneTimeInfo:(NSString *)config completion:(void (^)(NSError *error, NSString *config))completion{
    const char *config_char = [config cString];
    vcx_command_handle_t handle= [[VcxCallbacks sharedInstance] createCommandHandleFor:completion] ;
    vcx_error_t ret = vcx_agent_provision_async(handle, config_char, VcxWrapperCommonStringCallback);
    if( ret != 0 )
    {
        [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
        
        dispatch_async(dispatch_get_main_queue(), ^{
            NSLog(@"agentProvision: calling completion");
            completion([NSError errorFromVcxError: ret], false);
        });
    }
    
}

- (void)createConnectionWithInvite:(NSString *)invitationId
                inviteDetails:(NSString *)inviteDetails
             completion:(void (^)(NSError *error, NSString *credentialHandle)) completion
{
   vcx_error_t ret;

   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   ret = vcx_connection_create_with_invite(handle, invitationId, inviteDetails, VcxWrapperCommonCallback);
   if( ret != Success )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];

       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], nil);
       });
   }
}

- (void)acceptInvitation: (NSInteger *) connectionHandle
        connectionType: (NSString *) connectionType
            completion: (void (^)(NSError *error, NSString *inviteDetails)) completion
{
   vcx_error_t ret;
   
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   ret = vcx_connection_connect(handle, connectionHandle, connectionType, VcxWrapperCommonCallback)
   if( ret != Success )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret], nil);
       });
   }
}

- (void)updatePushToken: (NSInteger *) config
            completion: (void (^)(NSError *error)) completion
{
   vcx_error_t ret;
   
   vcx_command_handle_t handle = [[VcxCallbacks sharedInstance] createCommandHandleFor:completion];
   ret = vcx_agent_update_info(handle, config, VcxWrapperCommonCallback)
   if( ret != Success )
   {
       [[VcxCallbacks sharedInstance] deleteCommandHandleFor: handle];
       
       dispatch_async(dispatch_get_main_queue(), ^{
           completion([NSError errorFromVcxError: ret]);
       });
   }
}

@end
