package com.ppaass.protocol.vpn.message;

public class ProxyMessage extends Message<ProxyMessageBodyType, ProxyMessageBody> {
    public ProxyMessage(byte[] encryptionToken, EncryptionType encryptionType,
                        ProxyMessageBody body) {
        super(encryptionToken, encryptionType, body);
    }
}
