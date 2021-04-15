package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public abstract class MessageBody<T extends MessageBodyType> {
    private final String id;
    private final String userToken;
    private final String sourceHost;
    private final int sourcePort;
    private final String targetHost;
    private final int targetPort;
    private final T bodyType;
    private final byte[] data;
    private final String targetChannelId;
    private final String agentChannelId;

    public MessageBody(String id, String userToken, String sourceHost, int sourcePort, String targetHost,
                       int targetPort, T bodyType, String agentChannelId, String targetChannelId, byte[] data) {
        this.id = id;
        this.userToken = userToken;
        this.targetHost = targetHost;
        this.targetPort = targetPort;
        this.sourceHost = sourceHost;
        this.sourcePort = sourcePort;
        this.bodyType = bodyType;
        this.data = data;
        this.agentChannelId = agentChannelId;
        this.targetChannelId = targetChannelId;
    }

    public String getId() {
        return id;
    }

    public String getUserToken() {
        return userToken;
    }

    public String getTargetHost() {
        return targetHost;
    }

    public int getTargetPort() {
        return targetPort;
    }

    public int getSourcePort() {
        return sourcePort;
    }

    public String getSourceHost() {
        return sourceHost;
    }

    public T getBodyType() {
        return bodyType;
    }

    public byte[] getData() {
        return data;
    }

    public String getTargetChannelId() {
        return targetChannelId;
    }

    public String getAgentChannelId() {
        return agentChannelId;
    }

    @Override
    public String toString() {
        return "MessageBody{" +
                "id='" + id + '\'' +
                ", userToken='" + userToken + '\'' +
                ", sourceHost='" + sourceHost + '\'' +
                ", sourcePort=" + sourcePort +
                ", targetHost='" + targetHost + '\'' +
                ", targetPort=" + targetPort +
                ", bodyType=" + bodyType +
                ", data=" + Arrays.toString(data) +
                ", targetChannelId='" + targetChannelId + '\'' +
                ", agentChannelId='" + agentChannelId + '\'' +
                '}';
    }
}
