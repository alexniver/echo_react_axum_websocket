export const string2Arraybuffer = (str) => {
  var enc = new TextEncoder();
  return enc.encode(str);
}

export const arraybuffer2String = (buf) => {
  var enc = new TextDecoder("utf-8");
  return enc.decode(buf)
}

export function arraybufferConcat(arraybuffers) {
  let length = 0
  for (const v of arraybuffers)
    length += v.byteLength

  let buf = new Uint8Array(length)
  let offset = 0
  for (const v of arraybuffers) {
    const uint8view = new Uint8Array(v.buffer, v.byteOffset, v.byteLength)
    buf.set(uint8view, offset)
    offset += uint8view.byteLength
  }

  return buf
}
