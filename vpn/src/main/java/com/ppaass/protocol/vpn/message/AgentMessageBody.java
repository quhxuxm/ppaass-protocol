package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public class AgentMessageBody extends MessageBody<AgentMessageBodyType> {
    private final String agentInstanceId;
    private final String agentChannelId;

    public AgentMessageBody(String id, String agentInstanceId, String userToken, String sourceHost, int sourcePort,
                            String targetHost, int targetPort,
                            AgentMessageBodyType bodyType, String agentChannelId, byte[] data) {
        super(id, userToken, sourceHost, sourcePort, targetHost, targetPort, bodyType, data);
        this.agentInstanceId = agentInstanceId;
        this.agentChannelId = agentChannelId;
    }

    public String getAgentInstanceId() {
        return agentInstanceId;
    }

    public String getAgentChannelId() {
        return agentChannelId;
    }

    @Override
    public String toString() {
        return "AgentMessageBody{" +
                ", id='" + getId() + '\'' +
                "agentInstanceId='" + agentInstanceId + '\'' +
                ", userToken='" + getUserToken() + '\'' +
                ", targetHost='" + getTargetHost() + '\'' +
                ", targetPort=" + getTargetPort() +
                ", sourcePort=" + getSourcePort() +
                ", sourceHost='" + getSourceHost() + '\'' +
                ", bodyType=" + getBodyType() +
                ", agentChannelId=" + agentChannelId +
                ", data=" + Arrays.toString(getData()) +
                "} " + super.toString();
    }
}
