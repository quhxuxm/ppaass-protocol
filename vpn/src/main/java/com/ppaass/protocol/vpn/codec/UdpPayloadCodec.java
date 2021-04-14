package com.ppaass.protocol.vpn.codec;

import com.ppaass.protocol.vpn.payload.UdpPayload;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufUtil;
import io.netty.buffer.Unpooled;

import java.nio.charset.StandardCharsets;

public class UdpPayloadCodec {
    public static final UdpPayloadCodec INSTANCE = new UdpPayloadCodec();

    private UdpPayloadCodec() {
    }

    public UdpPayload decodeUdpPayload(byte[] udpPayloadBytesArray) {
        ByteBuf udpPayloadByteBuf = Unpooled.wrappedBuffer(udpPayloadBytesArray);
        UdpPayload result = new UdpPayload();
        int addrTypeLength = udpPayloadByteBuf.readInt();
        String addrTypeName = udpPayloadByteBuf.readCharSequence(addrTypeLength, StandardCharsets.UTF_8).toString();
        UdpPayload.AddrType addrType = UdpPayload.AddrType.valueOf(addrTypeName);
        result.setOriginalAddrType(addrType);
        int destinationAddressLength = udpPayloadByteBuf.readInt();
        String destinationAddress =
                udpPayloadByteBuf.readCharSequence(destinationAddressLength, StandardCharsets.UTF_8).toString();
        result.setOriginalDestinationAddress(destinationAddress);
        int destinationPort = udpPayloadByteBuf.readInt();
        result.setOriginalDestinationPort(destinationPort);
        int sourceAddressLength = udpPayloadByteBuf.readInt();
        String sourceAddress =
                udpPayloadByteBuf.readCharSequence(sourceAddressLength, StandardCharsets.UTF_8).toString();
        result.setOriginalSourceAddress(sourceAddress);
        int sourcePort = udpPayloadByteBuf.readInt();
        result.setOriginalSourcePort(sourcePort);
        int dataLength = udpPayloadByteBuf.readInt();
        byte[] data = new byte[dataLength];
        udpPayloadByteBuf.readBytes(data);
        result.setData(data);
        return result;
    }

    public byte[] encodeUdpPayload(UdpPayload udpPayload) {
        ByteBuf tempBuffer = Unpooled.buffer();
        UdpPayload.AddrType addrType = udpPayload.getOriginalAddrType();
        int addrTypeLength = addrType.name().length();
        byte[] addrTypeByteArray = addrType.name().getBytes(StandardCharsets.UTF_8);
        tempBuffer.writeInt(addrTypeLength);
        tempBuffer.writeBytes(addrTypeByteArray);
        String destinationAddress = udpPayload.getOriginalDestinationAddress();
        int destinationAddressLength = destinationAddress.length();
        byte[] destinationAddressByteArray = destinationAddress.getBytes(StandardCharsets.UTF_8);
        tempBuffer.writeInt(destinationAddressLength);
        tempBuffer.writeBytes(destinationAddressByteArray);
        tempBuffer.writeInt(udpPayload.getOriginalDestinationPort());
        String sourceAddress = udpPayload.getOriginalSourceAddress();
        int sourceAddressLength = sourceAddress.length();
        byte[] sourceAddressByteArray = sourceAddress.getBytes(StandardCharsets.UTF_8);
        tempBuffer.writeInt(sourceAddressLength);
        tempBuffer.writeBytes(sourceAddressByteArray);
        tempBuffer.writeInt(udpPayload.getOriginalSourcePort());
        tempBuffer.writeInt(udpPayload.getData().length);
        tempBuffer.writeBytes(udpPayload.getData());
        return ByteBufUtil.getBytes(tempBuffer);
    }
}
