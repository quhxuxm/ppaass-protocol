package com.ppaass.protocol.base.icmp;

public class IcmpPacketReader {
    public static final IcmpPacketReader INSTANCE = new IcmpPacketReader();

    private IcmpPacketReader() {
    }

    public IcmpPacket parse(byte[] input) {
        return null;
    }
}
