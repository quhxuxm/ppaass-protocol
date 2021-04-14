package com.ppaass.protocol.vpn.message;

public class ProxyMessageBody extends MessageBody<ProxyMessageBodyType> {
    private final String proxyInstanceId;

    public ProxyMessageBody(String id, String proxyInstanceId, String userToken, String targetHost, int targetPort,
                            ProxyMessageBodyType bodyType, byte[] data) {
        super(id, userToken, targetHost, targetPort, bodyType, data);
        this.proxyInstanceId = proxyInstanceId;
    }

    public String getProxyInstanceId() {
        return proxyInstanceId;
    }
}
