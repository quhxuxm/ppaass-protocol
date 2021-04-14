package com.ppaass.protocol.vpn.message;

public class AgentMessage extends Message<AgentMessageBodyType, AgentMessageBody> {
    public AgentMessage(byte[] encryptionToken, EncryptionType encryptionType,
                        AgentMessageBody body) {
        super(encryptionToken, encryptionType, body);
    }
}
