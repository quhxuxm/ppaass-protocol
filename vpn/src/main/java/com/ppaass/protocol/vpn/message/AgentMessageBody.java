package com.ppaass.protocol.vpn.message;

import java.util.Arrays;

public class AgentMessageBody extends MessageBody<AgentMessageBodyType> {
    private final String agentInstanceId;

    public AgentMessageBody(String id, String agentInstanceId, String userToken, String sourceHost, int sourcePort,
                            String targetHost, int targetPort,
                            AgentMessageBodyType bodyType, byte[] data) {
        super(id, userToken, sourceHost, sourcePort, targetHost, targetPort, bodyType, data);
        this.agentInstanceId = agentInstanceId;
    }

    public String getAgentInstanceId() {
        return agentInstanceId;
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
                ", data=" + Arrays.toString(getData()) +
                "} " + super.toString();
    }
}
