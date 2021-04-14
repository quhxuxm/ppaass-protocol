package com.ppaass.protocol.common.exception;

public class PpaassProtocolException extends RuntimeException {
    public PpaassProtocolException() {
    }

    public PpaassProtocolException(String message) {
        super(message);
    }

    public PpaassProtocolException(String message, Throwable cause) {
        super(message, cause);
    }

    public PpaassProtocolException(Throwable cause) {
        super(cause);
    }

    public PpaassProtocolException(String message, Throwable cause, boolean enableSuppression,
                                   boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
