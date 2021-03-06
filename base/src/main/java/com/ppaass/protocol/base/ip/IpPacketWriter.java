package com.ppaass.protocol.base.ip;

import com.ppaass.protocol.base.ChecksumUtil;
import com.ppaass.protocol.base.tcp.TcpPacket;
import com.ppaass.protocol.base.tcp.TcpPacketWriter;
import com.ppaass.protocol.base.udp.UdpPacket;
import com.ppaass.protocol.base.udp.UdpPacketWriter;

import java.nio.ByteBuffer;

public class IpPacketWriter {
    public static final IpPacketWriter INSTANCE = new IpPacketWriter();

    private IpPacketWriter() {
    }

    private int convertBoolean(boolean value) {
        if (value) {
            return 1;
        }
        return 0;
    }

    public byte[] write(IpPacket packet) {
        if (IpHeaderVersion.V4 != packet.getHeader().getVersion()) {
            throw new UnsupportedOperationException("Only support IpV4.");
        }
        byte[] headerToDoChecksum = this.writeIpV4HeaderWithGivenChecksum(packet, 0);
        int headerChecksum = ChecksumUtil.INSTANCE.checksum(headerToDoChecksum);
        byte[] headerBytes = this.writeIpV4HeaderWithGivenChecksum(packet, headerChecksum);
        IpV4Header ipV4Header = (IpV4Header) packet.getHeader();
        ByteBuffer resultBuffer = ByteBuffer.allocate(ipV4Header.getTotalLength());
        resultBuffer.put(headerBytes);
        if (IpDataProtocol.TCP == ipV4Header.getProtocol()) {
            resultBuffer.put(TcpPacketWriter.INSTANCE.write((TcpPacket) packet.getData(), ipV4Header));
            resultBuffer.flip();
            byte[] result = resultBuffer.array();
            resultBuffer.clear();
            return result;
        }
        if (IpDataProtocol.UDP == ipV4Header.getProtocol()) {
            resultBuffer.put(UdpPacketWriter.INSTANCE.write((UdpPacket) packet.getData(), ipV4Header));
            resultBuffer.flip();
            byte[] result = resultBuffer.array();
            resultBuffer.clear();
            return result;
        }
        throw new UnsupportedOperationException("Do not support other protocol.");
    }

    private byte[] writeIpV4HeaderWithGivenChecksum(IpPacket packet, int checksum) {
        IpV4Header ipV4Header = (IpV4Header) packet.getHeader();
        ByteBuffer byteBuffer = ByteBuffer.allocate(ipV4Header.getInternetHeaderLength() * 4);
        byte versionAndHeaderLength =
                (byte) (IpHeaderVersion.V4.getValue() << 4 | ipV4Header.getInternetHeaderLength());
        byteBuffer.put(versionAndHeaderLength);
        byte serviceType =
                (byte) ((ipV4Header.getDs().getImportance() << 5) |
                        this.convertBoolean(ipV4Header.getDs().isDelay()) << 4 |
                        this.convertBoolean(ipV4Header.getDs().isHighStream()) << 3 |
                        this.convertBoolean(ipV4Header.getDs().isHighAvailability()) << 2 |
                        this.convertBoolean(ipV4Header.getEcn().isLowCost()) << 1 | ipV4Header.getEcn().getResolve());
        byteBuffer.put(serviceType);
        byteBuffer.putShort((short) ipV4Header.getTotalLength());
        byteBuffer.putShort((short) ipV4Header.getIdentification());
        int flagsBit =
                ipV4Header.getFlags().getResolved() << 2 | this.convertBoolean(ipV4Header.getFlags().isDf()) << 1 |
                        this.convertBoolean(ipV4Header.getFlags().isMf());
        flagsBit = flagsBit << 13;
        int flagsAndOffset = flagsBit | ipV4Header.getFragmentOffset();
        byteBuffer.putShort((short) flagsAndOffset);
        byteBuffer.put((byte) ipV4Header.getTtl());
        byteBuffer.put((byte) ipV4Header.getProtocol().getValue());
        byteBuffer.putShort((short) checksum);
        byteBuffer.put(ipV4Header.getSourceAddress());
        byteBuffer.put(ipV4Header.getDestinationAddress());
        byteBuffer.put(ipV4Header.getOptions());
        byteBuffer.flip();
        byte[] result = byteBuffer.array();
        byteBuffer.clear();
        return result;
    }
}
