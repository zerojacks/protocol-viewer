use protocol_parser::auto_parse;

fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .replace(['-', ':'], "");
    
    if cleaned.is_empty() {
        return Err("数据内容不能为空".into());
    }
    if cleaned.len() % 2 != 0 {
        return Err("十六进制字符串长度必须是偶数".into());
    }
    
    let mut out = Vec::with_capacity(cleaned.len() / 2);
    let bytes = cleaned.as_bytes();
    for chunk in bytes.chunks(2) {
        let s = std::str::from_utf8(chunk).map_err(|_| "非法字符".to_string())?;
        let b = u8::from_str_radix(s, 16).map_err(|_| format!("非法十六进制: {s}"))?;
        out.push(b);
    }
    Ok(out)
}

fn main() {
    let hex_str = "68 96 02 96 02 68 4B FF FF FF FF FF FF 0A 0F 63 00 00 02 00 01 E3 F8 04 80 02 B0 3A 18 AB BE 81 E1 43 5E 7E 5E 85 39 17 2B 15 B6 70 88 42 6F 05 98 22 71 3D 1C E7 A3 D2 CE 7E 22 5F 7D A0 AB 75 E0 DB 62 A1 AF E7 5B 73 93 D0 9D 65 16 33 D4 DF 33 7D FB 2F 4E D9 0C 07 23 D4 FC 42 36 93 A7 20 6F 91 63 19 18 6F F5 F7 5B 59 A8 93 B4 8F 52 00 A1 6E 07 8F 58 32 97 05 74 D3 2A FE CC B4 4B 54 0F D1 8C AB 41 2F D8 85 3C 4C 53 6C 32 15 2A 43 18 61 B1 BE 37 CC E9 B5 B7 8D 76 4A 52 8A CD 71 B7 EE 79 82 07 BE 0E 1F F9 2A B9 A7 50 97 40 8C EE 03 F6 49 AC 17 8E 09 CD 68 5E AE A4 6E 20 B7 95 C9 93 3E A9 80 2F 4E DC 7A 4D 11 3A 2A 0E 52 48 D7 09 E1 57 F4 69 AD EE 29 DD FF BD C6 D7 AD FB E9 54 86 30 43 61 E7 02 52 28 58 2D 55 D5 A7 BD BC 20 C0 D0 C1 E6 68 58 D9 BC 33 6B 25 E7 B7 B9 22 61 8A BD 34 49 6B 81 DE 5E 45 B2 F6 E7 48 74 0E 4F 43 7B 31 E5 B3 A6 1B B4 09 08 22 27 F4 C8 50 16 12 C8 DE 27 2F 58 39 20 45 6C D2 A3 6E 46 D3 E8 71 64 B3 7C F7 D2 BB 45 20 F0 BA 4A 90 32 15 34 E8 BA 10 EC C2 78 E5 29 E6 3B BC DE DC 5C 8B 17 A6 9F 0E 09 72 B9 5D E6 AB 31 1D 90 97 28 FA 25 2C 54 98 1C 75 AB 31 84 73 3E 36 E7 DA 69 ED 06 57 E2 9F 81 4C A2 32 8E A4 A6 78 13 5F 85 3F 25 C6 41 30 F5 F1 FB AB BF BC 37 19 42 F8 0A 14 FC 45 91 2E 21 40 76 5C D1 EF DB B8 FF 9C C2 01 D6 E5 9B A6 11 9E 77 C2 23 12 5B DB E2 3B 91 B9 F5 FB C8 72 16 8A 5E 81 52 AE 6B 8C 9E 50 D9 2B 42 59 B9 8F 42 6B 3A 1A 32 01 B2 1B 0C B5 2E 32 A0 1C ED A5 48 76 15 0D 94 2F 83 5F 58 97 17 8C 42 62 E5 AB 02 73 2B 68 E9 D4 9E C2 F0 CB A8 11 A8 12 F5 DD C5 ED 35 97 AF 61 E9 FD E9 1A 9D 86 5B 35 51 87 7C 04 F7 71 2A 9F ED 4A 89 61 C6 79 53 43 BE 8A 91 F1 61 3B 3E E2 65 71 BF 42 38 73 1C C2 A6 A2 20 40 C1 7B BA 92 55 F7 00 35 AB 72 A4 2F 11 86 6A 77 BF 3B DD 2B 23 62 D4 D5 D6 1B C1 29 6E 75 6F 7F C9 78 6B B6 E4 6F 97 EF 7D 07 F6 05 CB DC 42 C4 D0 40 4C CE 06 E9 73 0D 50 5B 5A 16 1D ED D6 5B B8 EF D5 8B C7 05 36 E6 F7 B9 9B B5 B9 C9 4B 38 BD 47 98 5D 31 54 D2 8D D1 6A C8 FB 1D 29 B9 5A 4B 2F D7 05 64 36 E0 6C B1 E9 8D 3A 3A CD 09 B2 16 14 8A 89 9B 16";
    
    println!("解析文件传输报文...");
    
    match parse_hex_bytes(hex_str) {
        Ok(bytes) => {
            println!("十六进制解析成功，总字节数: {}", bytes.len());
            println!("前 20 字节: {:02X?}", &bytes[..20.min(bytes.len())]);
            
            // 检查帧格式
            if bytes.len() >= 6 {
                let frame_len = u16::from_le_bytes([bytes[1], bytes[2]]) as usize;
                let frame_len2 = u16::from_le_bytes([bytes[3], bytes[4]]) as usize;
                println!("帧长度域 L1 = 0x{:04X} ({} 字节)", frame_len, frame_len);
                println!("帧长度域 L2 = 0x{:04X} ({} 字节)", frame_len2, frame_len2);
                println!("实际总长度 = {} 字节 (应该是 {} + 8 = {})", 
                    bytes.len(), frame_len, frame_len + 8);
                    
                // 检查起始和结束符
                println!("起始符1 = 0x{:02X} (应为 0x68)", bytes[0]);
                println!("起始符2 = 0x{:02X} (应为 0x68)", bytes[5]);
                println!("结束符 = 0x{:02X} (应为 0x16)", bytes[bytes.len() - 1]);
                
                // 检查校验和
                if bytes.len() >= frame_len + 8 {
                    let user_data_start = 6;
                    let user_data_end = 6 + frame_len;
                    let user_data = &bytes[user_data_start..user_data_end];
                    let stored_cs = bytes[user_data_end];
                    
                    // 计算校验和：所有用户数据字节的算术和（不考虑进位）
                    let calculated_cs: u8 = user_data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
                    
                    println!("存储的校验和 CS = 0x{:02X}", stored_cs);
                    println!("计算的校验和 CS = 0x{:02X}", calculated_cs);
                    println!("校验和匹配: {}", stored_cs == calculated_cs);
                }
                    
                // 检查控制码和AFN
                if bytes.len() >= 14 {
                    println!("控制码 C = 0x{:02X}", bytes[6]);
                    println!("主站地址 MSA = 0x{:02X}", bytes[13]);
                    println!("应用功能码 AFN = 0x{:02X}", bytes[14]);
                }
            }
            
            // 检查协议检测
            println!("\n直接测试 Frame::decode:");
            match csg1209022::link::frame::Frame::decode(&bytes) {
                Ok((frame, consumed)) => {
                    println!("✅ Frame 解码成功!");
                    println!("  消耗字节: {}", consumed);
                    println!("  控制域: {:?}", frame.control);
                    println!("  地址域: {:?}", frame.address);
                    println!("  载荷长度: {} 字节", frame.payload.len());
                }
                Err(e) => {
                    println!("❌ Frame 解码失败: {:?}", e);
                }
            }
            
            println!("\n协议检测:");
            println!("is_csg1209022_frame: {}", protocol_parser::detect_protocol(&bytes).is_some());
            if let Some(protocol) = protocol_parser::detect_protocol(&bytes) {
                println!("检测到协议类型: {:?}", protocol);
            }
            
            match auto_parse(&bytes, None) {
                Ok((msg, consumed)) => {
                    println!("\n✅ 解析成功！");
                    println!("协议类型: {}", msg.protocol_name());
                    println!("消耗字节数: {}", consumed);
                    println!("解析结果: {:#?}", msg);
                }
                Err(e) => {
                    println!("\n❌ 自动解析失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("十六进制解析失败: {}", e);
        }
    }
}

fn print_tree(node: &protocol_parser::FieldValue, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!("{}{:?}", prefix, node);
}
