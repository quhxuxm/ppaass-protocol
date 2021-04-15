package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public class ProxyMessageBody extends MessageBody<ProxyMessageBodyType> {
    private final String proxyInstanceId;

    public ProxyMessageBody(String id, String proxyInstanceId, String userToken, String sourceHost, int sourcePort,
                            String targetHost, int targetPort,
                            ProxyMessageBodyType bodyType, String agentChannelId, String targetChannelId, byte[] data) {
        super(id, userToken, sourceHost, sourcePort, targetHost, targetPort, bodyType, agentChannelId, targetChannelId,
                data);
        this.proxyInstanceId = proxyInstanceId;
    }

    public String getProxyInstanceId() {
        return proxyInstanceId;
    }

    @Override
    public String toString() {
        return "ProxyMessageBody{" +
                "id='" + getId() + '\'' +
                ", userToken='" + getUserToken() + '\'' +
                ", targetHost='" + getTargetHost() + '\'' +
                ", targetPort=" + getTargetPort() +
                ", sourcePort=" + getSourcePort() +
                ", sourceHost='" + getSourceHost() + '\'' +
                ", bodyType=" + getBodyType() +
                ", data=" + Arrays.toString(getData()) +
                ", targetChannelId='" + getTargetChannelId() + '\'' +
                ", agentChannelId='" + getAgentChannelId() + '\'' +
                ", proxyInstanceId='" + proxyInstanceId + '\'' +
                "} ";
    }
}
