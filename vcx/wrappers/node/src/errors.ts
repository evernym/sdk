import { VCXCode } from './api/common'

// tslint:disable max-classes-per-file
export class ConnectionTimeoutError extends Error {}

export class VCXInternalError extends Error {
  public readonly vcxCode: number
  public readonly vcxFunction: string
  public readonly inheritedStackTraces: any[] = []

  constructor (code: number | Error, message: string, fn: string) {
    super(message)
    if (code instanceof Error) {
      if (code.stack) {
        this.inheritedStackTraces.push(code.stack)
      }
      if (code instanceof VCXInternalError) {
        this.vcxCode = code.vcxCode
        this.vcxFunction = code.vcxFunction
        this.inheritedStackTraces.unshift(...code.inheritedStackTraces)
        return this
      }
      this.vcxCode = VCXCode.UNKNOWN_ERROR
      this.vcxFunction = fn
      return this
    }
    this.vcxCode = code
    this.vcxFunction = fn
  }
}
