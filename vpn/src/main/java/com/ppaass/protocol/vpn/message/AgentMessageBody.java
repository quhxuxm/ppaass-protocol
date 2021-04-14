package com.ppaass.protocol.vpn.message;

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
}
