package com.evernym.sdk.vcx;

import java.util.HashMap;
import java.util.Map;

/**
 * Enumeration of error codes returned by the vcx SDK.
 */
public enum ErrorCode {

	/**
	 * Success
	 */
	Success(0),

	// Common errors


	/**
	 * IO Error
	 */
	CommonIOError(114);

	// Wallet errors


	private int value;
	private static Map<Integer, ErrorCode> map = new HashMap<Integer, ErrorCode>();
	
	private ErrorCode(int value) {

		this.value = value;
	}

	static {

		for (ErrorCode errorCode : ErrorCode.values()) {

			map.put(Integer.valueOf(errorCode.value), errorCode);
		}
	}

	/**
	 * Gets the ErrorCode that corresponds to the specified int value.
	 * 
	 * @param value	The integer to get the error code for.
	 * @return	The ErrorCode that corresponds to the specified integer.
	 */
	public static ErrorCode valueOf(int value) {

		return map.get(Integer.valueOf(value));
	}

	/**
	 * Gets the integer value for a specific ErrorCode.
	 * 
	 * @return The integer value of the ErrorCode.
	 */
	public int value() {

		return this.value;
	}
}
