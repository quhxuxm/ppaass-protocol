package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public class ProxyMessageBody extends MessageBody<ProxyMessageBodyType> {
    private final String proxyInstanceId;
    private final String targetChannelId;

    public ProxyMessageBody(String id, String proxyInstanceId, String userToken, String sourceHost, int sourcePort,
                            String targetHost, int targetPort,
                            ProxyMessageBodyType bodyType, String targetChannelId, byte[] data) {
        super(id, userToken, sourceHost, sourcePort, targetHost, targetPort, bodyType, data);
        this.proxyInstanceId = proxyInstanceId;
        this.targetChannelId = targetChannelId;
    }

    public String getProxyInstanceId() {
        return proxyInstanceId;
    }

    public String getTargetChannelId() {
        return targetChannelId;
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
                ", targetChannelId=" + targetChannelId +
                ", data=" + Arrays.toString(getData()) +
                "} " + super.toString();
    }
}
