package com.ppaass.protocol.base.tcp;

import com.ppaass.protocol.base.ip.IIpData;

public class TcpPacket implements IIpData {
    private final TcpHeader header;
    private final byte[] data;

    TcpPacket(TcpHeader header, byte[] data) {
        this.header = header;
        if (data == null) {
            this.data = new byte[]{};
        } else {
            this.data = data;
        }
    }

    public TcpHeader getHeader() {
        return header;
    }

    public byte[] getData() {
        return data;
    }

    @Override
    public String toString() {
        return "TcpPacket{" +
                "header=" + header +
                ", data size = " + data.length +
                '}';
    }
}
