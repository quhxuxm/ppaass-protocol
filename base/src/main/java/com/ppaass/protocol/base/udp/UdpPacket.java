package com.ppaass.protocol.base.udp;

import com.ppaass.protocol.base.ip.IIpData;

import java.util.Arrays;

public class UdpPacket implements IIpData {
    private final UdpHeader header;
    private final byte[] data;

    UdpPacket(UdpHeader header, byte[] data) {
        this.header = header;
        if (data == null) {
            this.data = new byte[]{};
        } else {
            this.data = data;
        }
    }

    public byte[] getData() {
        return data;
    }

    public UdpHeader getHeader() {
        return header;
    }

    @Override
    public String toString() {
        return "UdpPacket{" +
                "header=" + header +
                ", data=" + Arrays.toString(data) +
                '}';
    }
}
