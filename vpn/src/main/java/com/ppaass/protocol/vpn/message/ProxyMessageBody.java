package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public class ProxyMessageBody extends MessageBody<ProxyMessageBodyType> {
    private final String proxyInstanceId;

    public ProxyMessageBody(String id, String proxyInstanceId, String userToken, String sourceHost, int sourcePort,
                            String targetHost, int targetPort,
                            ProxyMessageBodyType bodyType, byte[] data) {
        super(id, userToken, sourceHost, sourcePort, targetHost, targetPort, bodyType, data);
        this.proxyInstanceId = proxyInstanceId;
    }

    public String getProxyInstanceId() {
        return proxyInstanceId;
    }

    @Override
    public String toString() {
        return "ProxyMessageBody{" +
                "id='" + getId() + '\'' +
                ", proxyInstanceId='" + proxyInstanceId + '\'' +
                ", userToken='" + getUserToken() + '\'' +
                ", targetHost='" + getTargetHost() + '\'' +
                ", targetPort=" + getTargetPort() +
                ", sourcePort=" + getSourcePort() +
                ", sourceHost='" + getSourceHost() + '\'' +
                ", bodyType=" + getBodyType() +
                ", data=" + Arrays.toString(getData()) +
                "} " + super.toString();
    }
}
