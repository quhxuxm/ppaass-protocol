package com.ppaass.protocol.vpn.codec;

import com.ppaass.protocol.vpn.cryptography.CryptographyUtil;
import com.ppaass.protocol.vpn.message.*;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufUtil;
import io.netty.buffer.Unpooled;
import io.netty.util.ReferenceCountUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.charset.StandardCharsets;

public class MessageCodec {
    private static final byte[] MAGIC_CODE = "__PPAASS__".getBytes(StandardCharsets.UTF_8);
    private static final Logger logger = LoggerFactory.getLogger(MessageCodec.class);
    public static final MessageCodec INSTANCE = new MessageCodec();

    private MessageCodec() {
    }

    private AgentMessageBodyType parseAgentMessageBodyType(int b) {
        for (AgentMessageBodyType e : AgentMessageBodyType.values()) {
            if (e.value() == b) {
                return e;
            }
        }
        return null;
    }

    private ProxyMessageBodyType parseProxyMessageBodyType(int b) {
        for (ProxyMessageBodyType e : ProxyMessageBodyType.values()) {
            if (e.value() == b) {
                return e;
            }
        }
        return null;
    }

    private EncryptionType parseEncryptionType(byte b) {
        for (EncryptionType e : EncryptionType.values()) {
            if (e.value() == b) {
                return e;
            }
        }
        return null;
    }

    private byte[] encryptMessageBody(byte[] messageBodyByteArrayBeforeEncrypt,
                                      EncryptionType messageBodyBodyEncryptionType,
                                      byte[] messageBodyEncryptionToken) {
        switch (messageBodyBodyEncryptionType) {
            case AES:
                return CryptographyUtil.INSTANCE.aesEncrypt(messageBodyEncryptionToken,
                        messageBodyByteArrayBeforeEncrypt);
            case BLOWFISH:
                return CryptographyUtil.INSTANCE.blowfishEncrypt(messageBodyEncryptionToken,
                        messageBodyByteArrayBeforeEncrypt);
            default:
                throw new IllegalArgumentException("Unsupported encryption type. ");
        }
    }

    private byte[] decryptMessageBody(byte[] messageBodyByteArrayBeforeDecrypt,
                                      EncryptionType messageBodyBodyEncryptionType,
                                      byte[] messageBodyEncryptionToken) {
        switch (messageBodyBodyEncryptionType) {
            case AES:
                return CryptographyUtil.INSTANCE.aesDecrypt(messageBodyEncryptionToken,
                        messageBodyByteArrayBeforeDecrypt);
            case BLOWFISH:
                return CryptographyUtil.INSTANCE.blowfishDecrypt(messageBodyEncryptionToken,
                        messageBodyByteArrayBeforeDecrypt);
            default:
                throw new IllegalArgumentException("Unsupported encryption type. ");
        }
    }

    private <T extends MessageBodyType> ByteBuf encodeAgentMessageBody(AgentMessageBody agentMessageBody,
                                                                       EncryptionType messageBodyBodyEncryptionType,
                                                                       byte[] messageBodyEncryptionToken) {
        ByteBuf tempBuffer = Unpooled.buffer();
        AgentMessageBodyType agentMessageBodyType = agentMessageBody.getBodyType();
        if (agentMessageBodyType == null) {
            tempBuffer.writeInt(0);
        } else {
            int bodyType = agentMessageBodyType.value();
            tempBuffer.writeInt(bodyType);
        }
        if (agentMessageBody.getId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] messageIdByteArray = agentMessageBody.getId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(messageIdByteArray.length);
            tempBuffer.writeBytes(messageIdByteArray);
        }
        if (agentMessageBody.getAgentInstanceId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] agentInstanceIdByteArray = agentMessageBody.getAgentInstanceId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(agentInstanceIdByteArray.length);
            tempBuffer.writeBytes(agentInstanceIdByteArray);
        }
        if (agentMessageBody.getUserToken() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] userTokenByteArray = agentMessageBody.getUserToken().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(userTokenByteArray.length);
            tempBuffer.writeBytes(userTokenByteArray);
        }
        if (agentMessageBody.getSourceHost() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] sourceAddressByteArray = agentMessageBody.getSourceHost().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(sourceAddressByteArray.length);
            tempBuffer.writeBytes(sourceAddressByteArray);
        }
        tempBuffer.writeInt(agentMessageBody.getSourcePort());
        if (agentMessageBody.getTargetHost() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetAddressByteArray = agentMessageBody.getTargetHost().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(targetAddressByteArray.length);
            tempBuffer.writeBytes(targetAddressByteArray);
        }
        tempBuffer.writeInt(agentMessageBody.getTargetPort());
        if (agentMessageBody.getAgentChannelId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] agentChannelIdByteArray = agentMessageBody.getAgentChannelId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(agentChannelIdByteArray.length);
            tempBuffer.writeBytes(agentChannelIdByteArray);
        }
        if (agentMessageBody.getTargetChannelId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetChannelIdByteArray = agentMessageBody.getTargetChannelId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(targetChannelIdByteArray.length);
            tempBuffer.writeBytes(targetChannelIdByteArray);
        }
        if (agentMessageBody.getData() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetOriginalData = agentMessageBody.getData();
            tempBuffer.writeInt(targetOriginalData.length);
            tempBuffer.writeBytes(targetOriginalData);
        }
        return Unpooled.wrappedBuffer(encryptMessageBody(
                tempBuffer.array(),
                messageBodyBodyEncryptionType,
                messageBodyEncryptionToken));
    }

    private <T extends MessageBodyType> ByteBuf encodeProxyMessageBody(ProxyMessageBody proxyMessageBody,
                                                                       EncryptionType messageBodyBodyEncryptionType,
                                                                       byte[] messageBodyEncryptionToken) {
        ByteBuf tempBuffer = Unpooled.buffer();
        ProxyMessageBodyType proxyMessageBodyType = proxyMessageBody.getBodyType();
        if (proxyMessageBodyType == null) {
            tempBuffer.writeInt(0);
        } else {
            int bodyType = proxyMessageBodyType.value();
            tempBuffer.writeInt(bodyType);
        }
        if (proxyMessageBody.getId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] messageIdByteArray = proxyMessageBody.getId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(messageIdByteArray.length);
            tempBuffer.writeBytes(messageIdByteArray);
        }
        if (proxyMessageBody.getProxyInstanceId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] proxyInstanceIdByteArray = proxyMessageBody.getProxyInstanceId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(proxyInstanceIdByteArray.length);
            tempBuffer.writeBytes(proxyInstanceIdByteArray);
        }
        if (proxyMessageBody.getUserToken() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] userTokenByteArray = proxyMessageBody.getUserToken().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(userTokenByteArray.length);
            tempBuffer.writeBytes(userTokenByteArray);
        }
        if (proxyMessageBody.getSourceHost() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] sourceAddressByteArray = proxyMessageBody.getSourceHost().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(sourceAddressByteArray.length);
            tempBuffer.writeBytes(sourceAddressByteArray);
        }
        tempBuffer.writeInt(proxyMessageBody.getSourcePort());
        if (proxyMessageBody.getTargetHost() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetAddressByteArray = proxyMessageBody.getTargetHost().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(targetAddressByteArray.length);
            tempBuffer.writeBytes(targetAddressByteArray);
        }
        tempBuffer.writeInt(proxyMessageBody.getTargetPort());
        if (proxyMessageBody.getAgentChannelId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] agentChannelIdByteArray = proxyMessageBody.getAgentChannelId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(agentChannelIdByteArray.length);
            tempBuffer.writeBytes(agentChannelIdByteArray);
        }
        if (proxyMessageBody.getTargetChannelId() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetChannelIdByteArray = proxyMessageBody.getTargetChannelId().getBytes(StandardCharsets.UTF_8);
            tempBuffer.writeInt(targetChannelIdByteArray.length);
            tempBuffer.writeBytes(targetChannelIdByteArray);
        }
        if (proxyMessageBody.getData() == null) {
            tempBuffer.writeInt(0);
        } else {
            byte[] targetOriginalData = proxyMessageBody.getData();
            tempBuffer.writeInt(targetOriginalData.length);
            tempBuffer.writeBytes(targetOriginalData);
        }
        return Unpooled.wrappedBuffer(encryptMessageBody(
                tempBuffer.array(),
                messageBodyBodyEncryptionType,
                messageBodyEncryptionToken));
    }

    private AgentMessageBody decodeAgentMessageBody(byte[] messageBytes,
                                                    EncryptionType messageBodyBodyEncryptionType,
                                                    byte[] messageBodyEncryptionToken) {
        byte[] messageBodyBytes =
                decryptMessageBody(messageBytes, messageBodyBodyEncryptionType, messageBodyEncryptionToken);
        ByteBuf messageBodyByteBuf = Unpooled.wrappedBuffer(messageBodyBytes);
        AgentMessageBodyType bodyType = null;
        int messageBodyTypeValue = messageBodyByteBuf.readInt();
        if (messageBodyTypeValue != 0) {
            bodyType = parseAgentMessageBodyType(messageBodyTypeValue);
        }
        String messageId = null;
        int messageIdLength = messageBodyByteBuf.readInt();
        if (messageIdLength > 0) {
            messageId =
                    messageBodyByteBuf.readCharSequence(messageIdLength, StandardCharsets.UTF_8).toString();
        }
        String agentInstanceId = null;
        int agentInstanceIdLength = messageBodyByteBuf.readInt();
        if (agentInstanceIdLength > 0) {
            agentInstanceId =
                    messageBodyByteBuf.readCharSequence(agentInstanceIdLength, StandardCharsets.UTF_8).toString();
        }
        String userToken = null;
        int userTokenLength = messageBodyByteBuf.readInt();
        if (userTokenLength > 0) {
            userToken =
                    messageBodyByteBuf.readCharSequence(userTokenLength, StandardCharsets.UTF_8).toString();
        }
        String sourceAddress = null;
        int sourceAddressLength = messageBodyByteBuf.readInt();
        if (sourceAddressLength > 0) {
            sourceAddress = messageBodyByteBuf.readCharSequence(sourceAddressLength,
                    StandardCharsets.UTF_8).toString();
        }
        int sourcePort = messageBodyByteBuf.readInt();
        String targetAddress = null;
        int targetAddressLength = messageBodyByteBuf.readInt();
        if (targetAddressLength > 0) {
            targetAddress = messageBodyByteBuf.readCharSequence(targetAddressLength,
                    StandardCharsets.UTF_8).toString();
        }
        int targetPort = messageBodyByteBuf.readInt();
        int agentChannelIdLength = messageBodyByteBuf.readInt();
        String agentChannelId = null;
        if (agentChannelIdLength > 0) {
            agentChannelId = messageBodyByteBuf.readCharSequence(agentChannelIdLength,
                    StandardCharsets.UTF_8).toString();
        }
        int targetChannelIdLength = messageBodyByteBuf.readInt();
        String targetChannelId = null;
        if (targetChannelIdLength > 0) {
            targetChannelId = messageBodyByteBuf.readCharSequence(targetChannelIdLength,
                    StandardCharsets.UTF_8).toString();
        }
        int originalDataLength = messageBodyByteBuf.readInt();
        byte[] originalData = null;
        if (originalDataLength > 0) {
            originalData = new byte[originalDataLength];
            messageBodyByteBuf.readBytes(originalData);
        }
        return new AgentMessageBody(messageId, agentInstanceId, userToken, sourceAddress, sourcePort, targetAddress,
                targetPort, bodyType, agentChannelId, targetChannelId,
                originalData);
    }

    private ProxyMessageBody decodeProxyMessageBody(byte[] messageBytes,
                                                    EncryptionType messageBodyBodyEncryptionType,
                                                    byte[] messageBodyEncryptionToken) {
        byte[] messageBodyBytes =
                decryptMessageBody(messageBytes, messageBodyBodyEncryptionType, messageBodyEncryptionToken);
        ByteBuf messageBodyByteBuf = Unpooled.wrappedBuffer(messageBodyBytes);
        ProxyMessageBodyType bodyType = null;
        int messageBodyTypeValue = messageBodyByteBuf.readInt();
        if (messageBodyTypeValue > 0) {
            bodyType = parseProxyMessageBodyType(messageBodyTypeValue);
        }
        String messageId = null;
        int messageIdLength = messageBodyByteBuf.readInt();
        if (messageIdLength > 0) {
            messageId = messageBodyByteBuf.readCharSequence(messageIdLength, StandardCharsets.UTF_8).toString();
        }
        String proxyInstanceId = null;
        int proxyInstanceIdLength = messageBodyByteBuf.readInt();
        if (proxyInstanceIdLength > 0) {
            proxyInstanceId =
                    messageBodyByteBuf.readCharSequence(proxyInstanceIdLength, StandardCharsets.UTF_8).toString();
        }
        String userToken = null;
        int userTokenLength = messageBodyByteBuf.readInt();
        if (userTokenLength > 0) {
            userToken =
                    messageBodyByteBuf.readCharSequence(userTokenLength, StandardCharsets.UTF_8).toString();
        }
        String sourceAddress = null;
        int sourceAddressLength = messageBodyByteBuf.readInt();
        if (sourceAddressLength > 0) {
            sourceAddress = messageBodyByteBuf.readCharSequence(sourceAddressLength,
                    StandardCharsets.UTF_8).toString();
        }
        int sourcePort = messageBodyByteBuf.readInt();
        String targetAddress = null;
        int targetAddressLength = messageBodyByteBuf.readInt();
        if (targetAddressLength > 0) {
            targetAddress = messageBodyByteBuf.readCharSequence(targetAddressLength,
                    StandardCharsets.UTF_8).toString();
        }
        int targetPort = messageBodyByteBuf.readInt();
        int agentChannelIdLength = messageBodyByteBuf.readInt();
        String agentChannelId = null;
        if (agentChannelIdLength > 0) {
            agentChannelId = messageBodyByteBuf.readCharSequence(agentChannelIdLength,
                    StandardCharsets.UTF_8).toString();
        }
        int targetChannelIdLength = messageBodyByteBuf.readInt();
        String targetChannelId = null;
        if (targetChannelIdLength > 0) {
            targetChannelId = messageBodyByteBuf.readCharSequence(targetChannelIdLength,
                    StandardCharsets.UTF_8).toString();
        }
        int originalDataLength = messageBodyByteBuf.readInt();
        byte[] originalData = null;
        if (originalDataLength > 0) {
            originalData = new byte[originalDataLength];
            messageBodyByteBuf.readBytes(originalData);
        }
        return new ProxyMessageBody(messageId, proxyInstanceId, userToken, sourceAddress, sourcePort, targetAddress,
                targetPort, bodyType, agentChannelId, targetChannelId,
                originalData);
    }

    /**
     * Encode a message to byte buffer.
     *
     * @param message The message to encode.
     * @param output  The output byte buffer
     */
    public <T extends MessageBodyType> void encodeAgentMessage(AgentMessage message,
                                                               ByteBuf output) {
        output.writeBytes(MAGIC_CODE);
        byte[] originalMessageBodyEncryptionToken = message.getEncryptionToken();
        byte[] encryptedMessageBodyEncryptionToken =
                CryptographyUtil.INSTANCE.rsaEncrypt(originalMessageBodyEncryptionToken);
        output.writeInt(encryptedMessageBodyEncryptionToken.length);
        output.writeBytes(encryptedMessageBodyEncryptionToken);
        output.writeByte(message.getEncryptionType().value());
        ByteBuf bodyByteBuf = encodeAgentMessageBody(message.getBody(),
                message.getEncryptionType(),
                originalMessageBodyEncryptionToken);
        output.writeBytes(bodyByteBuf);
    }

    /**
     * Encode a message to byte buffer.
     *
     * @param message The message to encode.
     * @param output  The output byte buffer
     */
    public <T extends MessageBodyType> void encodeProxyMessage(ProxyMessage message,
                                                               ByteBuf output) {
        output.writeBytes(MAGIC_CODE);
        byte[] originalMessageBodyEncryptionToken = message.getEncryptionToken();
        byte[] encryptedMessageBodyEncryptionToken =
                CryptographyUtil.INSTANCE.rsaEncrypt(originalMessageBodyEncryptionToken);
        output.writeInt(encryptedMessageBodyEncryptionToken.length);
        output.writeBytes(encryptedMessageBodyEncryptionToken);
        output.writeByte(message.getEncryptionType().value());
        ByteBuf bodyByteBuf = encodeProxyMessageBody(message.getBody(),
                message.getEncryptionType(),
                originalMessageBodyEncryptionToken);
        output.writeBytes(bodyByteBuf);
    }

    /**
     * Decode agent message from input byte buffer.
     *
     * @param input The input byte buffer.
     * @return The agent message
     */
    public AgentMessage decodeAgentMessage(ByteBuf input) {
        ByteBuf magicCodeByteBuf = input.readBytes(MAGIC_CODE.length);
        if (magicCodeByteBuf.compareTo(Unpooled.wrappedBuffer(MAGIC_CODE)) != 0) {
            logger.error(
                    "Incoming agent message is not follow Ppaass protocol, incoming message is:\n{}\n"
                    , ByteBufUtil.prettyHexDump(input));
            throw new IllegalStateException("Incoming message is not follow Ppaass protocol.");
        }
        ReferenceCountUtil.release(magicCodeByteBuf);
        int encryptedMessageBodyEncryptionTokenLength = input.readInt();
        byte[] encryptedMessageBodyEncryptionToken = new byte[encryptedMessageBodyEncryptionTokenLength];
        input.readBytes(encryptedMessageBodyEncryptionToken);
        byte[] messageBodyEncryptionToken =
                CryptographyUtil.INSTANCE.rsaDecrypt(encryptedMessageBodyEncryptionToken);
        EncryptionType messageBodyEncryptionType = parseEncryptionType(input.readByte());
        if (messageBodyEncryptionType == null) {
            throw new IllegalStateException(
                    "Can not parse encryption type from the message.");
        }
        byte[] messageBodyByteArray = new byte[input.readableBytes()];
        input.readBytes(messageBodyByteArray);
        return new AgentMessage(messageBodyEncryptionToken,
                messageBodyEncryptionType,
                decodeAgentMessageBody(
                        messageBodyByteArray,
                        messageBodyEncryptionType,
                        messageBodyEncryptionToken));
    }

    /**
     * Decode proxy message from input byte buffer.
     *
     * @param input The input byte buffer.
     * @return The proxy message
     */
    public ProxyMessage decodeProxyMessage(ByteBuf input) {
        ByteBuf magicCodeByteBuf = input.readBytes(MAGIC_CODE.length);
        if (magicCodeByteBuf.compareTo(Unpooled.wrappedBuffer(MAGIC_CODE)) != 0) {
            logger.error(
                    "Incoming proxy message is not follow Ppaass protocol, incoming message is:\n${}\n",
                    ByteBufUtil.prettyHexDump(input)
            );
            throw new IllegalStateException("Incoming message is not follow Ppaass protocol.");
        }
        ReferenceCountUtil.release(magicCodeByteBuf);
        int encryptedMessageBodyEncryptionTokenLength = input.readInt();
        byte[] encryptedMessageBodyEncryptionToken = new byte[encryptedMessageBodyEncryptionTokenLength];
        input.readBytes(encryptedMessageBodyEncryptionToken);
        byte[] messageBodyEncryptionToken =
                CryptographyUtil.INSTANCE.rsaDecrypt(encryptedMessageBodyEncryptionToken);
        EncryptionType messageBodyEncryptionType = parseEncryptionType(input.readByte());
        if (messageBodyEncryptionType == null) {
            throw new IllegalStateException(
                    "Can not parse encryption type from the message.");
        }
        byte[] messageBodyByteArray = new byte[input.readableBytes()];
        input.readBytes(messageBodyByteArray);
        return new ProxyMessage(messageBodyEncryptionToken,
                messageBodyEncryptionType,
                decodeProxyMessageBody(
                        messageBodyByteArray,
                        messageBodyEncryptionType,
                        messageBodyEncryptionToken));
    }
}
