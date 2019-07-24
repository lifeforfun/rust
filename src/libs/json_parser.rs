use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::Chars;
use std::str::FromStr;

type F64 = f64;
type I64 = i64;

#[derive(Debug, PartialEq, PartialOrd)]
enum Number {
    I64(I64),
    F64(F64),
}

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug)]
struct ParserIter<'a> {
    cursor: Option<char>,
    s: &'a mut Chars<'a>,
}

impl<'a> ParserIter<'a> {
    fn new(code: &'a mut Chars<'a>) -> ParserIter<'a> {
        ParserIter {
            cursor: code.next(),
            s: code,
        }
    }

    fn next_char(&mut self, trim_whitespace: bool) -> Option<char> {
        self.cursor = self.s.next();
        if trim_whitespace {
            while let Some(c) = self.cursor {
                match c {
                    '\n' | '\r' | '\t' | ' ' => self.cursor = self.s.next(),
                    _ => break,
                };
            }
        }
        self.cursor
    }

    fn parse(&mut self) -> Result<Value, String> {
        self.trim_whitespaces();
        if let Some(Ok(v)) = self.parse_literal() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_string() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_number() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_array() {
            Ok(v)
        } else if let Some(Ok(v)) = self.parse_object() {
            Ok(v)
        } else {
            Err(format!("parse json error"))
        }
    }

    /// 返回之后指针指向最后一个字符
    fn get_str(&mut self, len: usize) -> Vec<char> {
        let mut vc = vec![];
        loop {
            if let Some(c) = self.cursor {
                vc.push(c);
            } else {
                break;
            }
            if len == vc.len() {
                break;
            }
            self.next_char(false);
        }
        vc
    }

    fn trim_whitespaces(&mut self) {
        while let Some(c) = self.cursor {
            match c {
                '\n' | '\r' | '\t' | ' ' => self.next_char(true),
                _ => break,
            };
        }
    }

    fn parse_literal(&mut self) -> Option<Result<Value, String>> {
        let mut literal = None;

        if let Some(c) = self.cursor {
            match c {
                't' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s == "true" {
                        literal = Some(Ok(Value::Bool(true)));
                    } else {
                        return Some(Err(format!(
                            "parse bool error: expect 'true' found {:?}",
                            s
                        )));
                    }
                }
                'f' => {
                    let s = self.get_str(5).into_iter().collect::<String>();
                    if s == "false" {
                        literal = Some(Ok(Value::Bool(false)));
                    } else {
                        return Some(Err(format!(
                            "parse bool error: expect 'false' found {:?}",
                            s
                        )));
                    }
                }
                'n' => {
                    let s = self.get_str(4).into_iter().collect::<String>();
                    if s == "null" {
                        literal = Some(Ok(Value::Null));
                    } else {
                        return Some(Err(format!("parse error: expect 'null' found {}", s)));
                    }
                }
                _ => {}
            }
        }
        if let Some(_) = literal {
            // 确保跳出后指向下一个字符
            self.next_char(true);
        }
        literal
    }

    fn parse_number(&mut self) -> Option<Result<Value, String>> {
        let mut nv = vec![];
        while let Some(c) = self.cursor {
            match c {
                '0'..='9' | '+' | '-' | 'e' | 'E' | '.' => {
                    nv.push(c);
                    self.next_char(false);
                }
                _ => break,
            }
        }
        if nv.len() == 0 {
            return None;
        }
        // 匹配数字结束后已经指向下一个字符，所以只去除空白符即可
        self.trim_whitespaces();
        let nstring = nv.into_iter().collect::<String>();
        let nstr = &nstring[..];
        if let Some(_) = nstr.find('.') {
            Some(
                F64::from_str(nstr)
                    .map_err(|e| -> String { format!("parse f64 error: {}", e) })
                    .map(|v| -> Value { Value::Number(Number::F64(v)) }),
            )
        } else {
            Some(
                I64::from_str(nstr)
                    .map_err(|e| -> String { format!("parse i64 error: {}", e) })
                    .map(|v| -> Value { Value::Number(Number::I64(v)) }),
            )
        }
    }

    fn parse_string(&mut self) -> Option<Result<Value, String>> {
        let mut name_start = false;
        let mut nv = vec![];
        let c = self.cursor?;
        if c != '"' {
            return Some(Err(format!("unexpect character {}, expect '\"'", c)));
        }
        while let Some(c) = self.cursor {
            match c {
                '\\' => {
                    let next = self.next_char(false)?;
                    match next {
                        '"' => {
                            nv.push('"');
                        }
                        '\\' => {
                            nv.push('\\');
                        }
                        '/' => {
                            nv.push('/');
                        }
                        'b' => {
                            nv.push('\x08');
                        }
                        'f' => {
                            nv.push('\x0C');
                        }
                        'n' => {
                            nv.push('\n');
                        }
                        'r' => {
                            nv.push('\r');
                        }
                        't' => {
                            nv.push('\t');
                        }
                        'u' => {
                            self.next_char(false);
                            let s = self.get_str(4).into_iter().collect::<String>();
                            if let Ok(i) = u32::from_str_radix(&s[..], 16) {
                                if let Ok(ic) = char::try_from(i) {
                                    nv.push(ic);
                                }
                            } else {
                                return Some(Err(format!(
                                    "escape unicode characters error : {}",
                                    s
                                )));
                            }
                        }
                        _ => {
                            return Some(Err(format!("unexpected escaped character {}", next)));
                        }
                    };
                }
                '"' => {
                    if name_start {
                        // 确保跳出后指向下一个字符
                        self.next_char(true);
                        return Some(Ok(Value::String(nv.into_iter().collect::<String>())));
                    }
                    name_start = true;
                }
                other => {
                    if !name_start {
                        return Some(Err(format!("parse string error , string start not found")));
                    }
                    nv.push(other);
                }
            };
            self.next_char(false);
        }
        None
    }

    fn parse_array(&mut self) -> Option<Result<Value, String>> {
        let c = self.cursor?;
        if c != '[' {
            return Some(Err(format!(
                "unexpected array start, expect [ but found {}",
                c
            )));
        }
        let mut arr_start = false;
        let mut nv = vec![];
        while let Some(c) = self.cursor {
            match c {
                ']' => {
                    if !arr_start {
                        return Some(Err(format!("parse array error")));
                    }
                    // 确保跳出后指向下一个字符
                    self.next_char(true);
                    return Some(Ok(Value::Array(nv)));
                }
                ',' | '[' => {
                    if c == '[' {
                        arr_start = true;
                    } else {
                        if !arr_start {
                            return Some(Err(format!("parse array error")));
                        }
                    }
                    self.next_char(true);
                    if let Some(Ok(v)) = self.parse_array() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_literal() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_number() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_string() {
                        nv.push(v);
                    } else if let Some(Ok(v)) = self.parse_object() {
                        nv.push(v);
                    } else {
                        return Some(Err(format!(
                            "parse array error: unexpected {}",
                            self.cursor?
                        )));
                    }
                }
                other => {
                    return Some(Err(format!("unexpected character '{}'", other)));
                }
            }
            self.trim_whitespaces();
        }
        None
    }

    fn parse_object(&mut self) -> Option<Result<Value, String>> {
        let c = self.cursor?;
        if c != '{' {
            return Some(Err(format!(
                "parse object failed: unexpected character {}",
                c
            )));
        }

        let mut ob_start = false;
        let mut obj = HashMap::new();
        while let Some(c) = self.cursor {
            match c {
                ',' | '{' => {
                    if c == '{' {
                        ob_start = true;
                    }
                    self.next_char(true);
                    if let Some(Ok(Value::String(name))) = self.parse_string() {
                        let colon = self.cursor?;
                        if colon != ':' {
                            return Some(Err(format!("expect colon but found {}", colon)));
                        }
                        self.next_char(true);
                        if let Some(Ok(v)) = self.parse_object() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_array() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_literal() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_string() {
                            obj.insert(name, v);
                        } else if let Some(Ok(v)) = self.parse_number() {
                            obj.insert(name, v);
                        } else {
                            return Some(Err(format!(
                                "parse object error, unexpected {}",
                                self.cursor?
                            )));
                        }
                    }
                }
                '}' => {
                    if !ob_start {
                        return Some(Err(format!("unexpected object close brace")));
                    }
                    // 确保跳出后指向下一个字符
                    self.next_char(true);
                    return Some(Ok(Value::Object(obj)));
                }
                c => return Some(Err(format!("unexpected character {}", c))),
            }
            self.trim_whitespaces();
        }
        None
    }
}

pub fn test() {
    let data = r#"
         {
	"status": {
		"code": 10000,
		"message": "成功",
		"alert": "",
		"server_time": 1563776832
	},
	"data": {
		"id": 600434,
		"type": 1,
		"name": "绿豆汤",
		"description": "青砖黛瓦、古城风味。推门而入 ，仿佛穿越千年时光，感受到老成都的市井热闹。\r\n青砖黛瓦、古城风味。推门而入 ，仿佛穿越千年时光，感受到老成都的市井热闹。\r\n青砖黛瓦、古城风味。推门而入 ，仿佛穿越千年时光，感受到老成都的市井热闹。a\r\n\r\n四四方方的木桌椅，没有太多雕饰，却带着满满的厚重感；暖黄色的灯光，趁着蒸腾的热气，愈发让人沉醉~\r\n\r\n还有简约私密的小包厢哦~几人围坐一桌，觥筹交错，吃的就是这一种氛围~\r\na\r\n\r\n秘制锅底，热辣鲜香\r\n\r\n大有玄机的红油锅底，融合了90多种火锅底料秘方，统一炒制，再从成都总店打飞的直运，保证你吃到的锅底都是新鲜的！\r\n\r\na\r\n\r\n蜀地的花椒、辣椒和豆瓣，泼上够分量的牛油，保证了老道川味火锅的麻辣鲜香。\r\na\r\n\r\n更不能少的，是这一罐小龙坎特制油碟！清爽不腻，清火解辣，热热的夏天吃火锅，也能吃出清爽感~\r\na\r\n\r\n明星单品，越吃越爽\r\n\r\n大刀毛肚\r\n\r\n用最新鲜的牛瓣胃，再除去边角，只选取厚薄均匀的部分，最后切巴掌大小铺在上，一根根竖刺清晰可见，超赞~\r\na\r\n\r\n在锅中涮个七上八下即可，爽脆弹牙“咯叽咯叽”停不下来。\r\na\r\n\r\n冰球鹅肠\r\n\r\n打飞的来的鹅肠，造型美颜极了。圆滚滚的冰球包裹着爽脆的鹅肠，完美的保持住了鹅肠的鲜度与口感。\r\na\r\n\r\n超长一根的鹅肠，白里透粉缓缓下锅，烫至微卷入口超级过瘾。\r\na\r\n\r\n麻辣牛肉\r\n\r\n嗜辣星人必点的麻辣牛肉！细碎的辣椒将牛肉360°全方面包裹住，足足腌制了5小时才能上桌。\r\na\r\n\r\n即使在热辣的锅中久煮，辣椒还是紧紧的粘住。辣上加辣，却也越吃越香。\r\na\r\n\r\n玫瑰丸子\r\n\r\n小龙坎独家秘制，采用新鲜食用玫瑰、猪五花肉纯手工打制二十来分钟才成的丸子，匠心在里面有没有！\r\na\r\n\r\n吸收麻辣之余还能吃出一缕玫瑰的花香，撩妹必点！送花又送肉，绝对够浪漫~\r\na\r\n\r\n养生肥牛\r\n\r\n超大片的肥牛包着一整块秋葵，口感层次绝对让你惊艳！一口咬下，肥牛的鲜嫩，秋葵的脆爽美味又养生。\r\na\r\n\r\n红糖糍粑\r\n\r\n最后来份小甜点，炸得外脆里嫩的糍粑趁热吃口感最佳，裹上些红糖汁香甜软糯，正好解辣。\r\n\r\na\r\n\r\n四四方方的木桌椅，没有太多雕饰，却带着满满的厚重感；暖黄色的灯光，趁着蒸腾的热气，愈发让人沉醉~\r\na\r\na\r\na\r\n\r\n还有简约私密的小包厢哦~几人围坐一桌，觥筹交错，吃的就是这一种氛围~\r\na\r\n\r\n秘制锅底，热辣鲜香\r\n\r\n大有玄机的红油锅底，融合了90多种火锅底料秘方，统一炒制，再从成都总店打飞的直运，保证你吃到的锅底都是新鲜的！\r\n\r\na\r\n\r\n蜀地的花椒、辣椒和豆瓣，泼上够分量的牛油，保证了老道川味火锅的麻辣鲜香。\r\na\r\n\r\n更不能少的，是这一罐小龙坎特制油碟！清爽不腻，清火解辣，热热的夏天吃火锅，也能吃出清爽感~\r\na\r\n\r\n明星单品，越吃越爽\r\n\r\n大刀毛肚\r\n\r\n用最新鲜的牛瓣胃，再除去边角，只选取厚薄均匀的部分，最后切巴掌大小铺在上，一根根竖刺清晰可见，超赞~\r\na\r\n\r\n在锅中涮个七上八下即可，爽脆弹牙“咯叽咯叽”停不下来。\r\na\r\n\r\n冰球鹅肠\r\n\r\n打飞的来的鹅肠，造型美颜极了。圆滚滚的冰球包裹着爽脆的鹅肠，完美的保持住了鹅肠的鲜度与口感。\r\na\r\n\r\n超长一根的鹅肠，白里透粉缓缓下锅，烫至微卷入口超级过瘾。\r\na\r\n\r\n麻辣牛肉\r\n\r\n嗜辣星人必点的麻辣牛肉！细碎的辣椒将牛肉360°全方面包裹住，足足腌制了5小时才能上桌。\r\na\r\n\r\n即使在热辣的锅中久煮，辣椒还是紧紧的粘住。辣上加辣，却也越吃越香。\r\na\r\n\r\n玫瑰丸子\r\n\r\n小龙坎独家秘制，采用新鲜食用玫瑰、猪五花肉纯手工打制二十来分钟才成的丸子，匠心在里面有没有！\r\na\r\n\r\n吸收麻辣之余还能吃出一缕玫瑰的花香，撩妹必点！送花又送肉，绝对够浪漫~\r\na\r\n\r\n养生肥牛\r\n\r\n超大片的肥牛包着一整块秋葵，口感层次绝对让你惊艳！一口咬下，肥牛的鲜嫩，秋葵的脆爽美味又养生。\r\na\r\n\r\n红糖糍粑\r\n\r\n最后来份小甜点，炸得外脆里嫩的糍粑趁热吃口感最佳，裹上些红糖汁香甜软糯，正好解辣。\r\n\r\n\r\na\r\n\r\n四四方方的木桌椅，没有太多雕饰，却带着满满的厚重感；暖黄色的灯光，趁着蒸腾的热气，愈发让人沉醉~\r\na\r\na\r\na\r\n\r\n还有简约私密的小包厢哦~几人围坐一桌，觥筹交错，吃的就是这一种氛围~\r\na\r\n\r\n秘制锅底，热辣鲜香\r\n\r\n大有玄机的红油锅底，融合了90多种火锅底料秘方，统一炒制，再从成都总店打飞的直运，保证你吃到的锅底都是新鲜的！\r\n\r\na\r\n\r\n蜀地的花椒、辣椒和豆瓣，泼上够分量的牛油，保证了老道川味火锅的麻辣鲜香。\r\na\r\n\r\n更不能少的，是这一罐小龙坎特制油碟！清爽不腻，清火解辣，热热的夏天吃火锅，也能吃出清爽感~\r\na\r\n\r\n明星单品，越吃越爽\r\n\r\n大刀毛肚\r\n\r\n用最新鲜的牛瓣胃，再除去边角，只选取厚薄均匀的部分，最后切巴掌大小铺在上，一根根竖刺清晰可见，超赞~\r\na\r\n\r\n在锅中涮个七上八下即可，爽脆弹牙“咯叽咯叽”停不下来。\r\na\r\n\r\n冰球鹅肠\r\n\r\n打飞的来的鹅肠，造型美颜极了。圆滚滚的冰球包裹着爽脆的鹅肠，完美的保持住了鹅肠的鲜度与口感。\r\na\r\n\r\n超长一根的鹅肠，白里透粉缓缓下锅，烫至微卷入口超级过瘾。\r\na\r\n\r\n麻辣牛肉\r\n\r\n嗜辣星人必点的麻辣牛肉！细碎的辣椒将牛肉360°全方面包裹住，足足腌制了5小时才能上桌。\r\na\r\n\r\n即使在热辣的锅中久煮，辣椒还是紧紧的粘住。辣上加辣，却也越吃越香。\r\na\r\n\r\n玫瑰丸子\r\n\r\n小龙坎独家秘制，采用新鲜食用玫瑰、猪五花肉纯手工打制二十来分钟才成的丸子，匠心在里面有没有！\r\na\r\n\r\n吸收麻辣之余还能吃出一缕玫瑰的花香，撩妹必点！送花又送肉，绝对够浪漫~\r\na\r\n\r\n养生肥牛\r\n\r\n超大片的肥牛包着一整块秋葵，口感层次绝对让你惊艳！一口咬下，肥牛的鲜嫩，秋葵的脆爽美味又养生。\r\na\r\n\r\n红糖糍粑\r\n\r\n最后来份小甜点，炸得外脆里嫩的糍粑趁热吃口感最佳，裹上些红糖汁香甜软糯，正好解辣。\r\na",
		"menus": [{
			"name": "主食",
			"items": [{
				"name": "米饭",
				"price": "2.00"
			}, {
				"name": "粥",
				"price": "5.78"
			}]
		}, {
			"name": "饮料",
			"items": [{
				"name": "大发发",
				"price": "111.00"
			}, {
				"name": "阿发发",
				"price": "453.00"
			}, {
				"name": "afa",
				"price": "34.99"
			}]
		}],
		"images": ["http:\/\/img-agc.iqianggou.com\/85ea146d0879d7a8d0b5df588ec7f98b", "http:\/\/img-agc.iqianggou.com\/2b9cf5e79be48f22875d0124bcbed807", "http:\/\/img-agc.iqianggou.com\/c4868d789471ac4998f775c116ea736d", "http:\/\/img-agc.iqianggou.com\/6bd71a5c4ceaa99d24e5cda186f981cf"],
		"start_price": 2,
		"floor_price": 1,
		"market_price": 2,
		"unlock_price": 1,
		"current_price": 0.01,
		"is_new": false,
		"left": 990,
		"tips": "1，不与店内其他优惠活动同享。\n2，商品图片由商户提供，仅供参考；实际商品以实物为准，如有不符，请于商户当面协商解决。\n3，请在点单前出示爱抢购订单，否则商家有权要求以正常价格买单。",
		"freeze_period": 1,
		"need_book": 1,
		"allow_take_out": 2,
		"weight": 7,
		"redeem_period": 7,
		"refund_type": 1,
		"delivery_type": 1,
		"source_type": 1,
		"payment_way": [256, 64, 32, 8, 4, 2, 1],
		"start_time": 1534788000,
		"end_time": 1585411199,
		"enabled": true,
		"brand": "松乃家日式猪排专门店",
		"bargain": false,
		"bargain_range": 0.5,
		"vendor_item_online": true,
		"like": 1,
		"branches": [{
			"id": 69201,
			"name": "松乃家日式猪排专门店(日月光店)",
			"logo": "",
			"logo_small": "http:\/\/img-agc.iqianggou.com\/3ae533ac8065fa2d703519550e9f0463",
			"description": "",
			"redeem_type": [8, 4, 2, 1],
			"address": "上海市黄浦区日月光B2层XJH-B2-31号",
			"address_short": "日月光B2层XJH-B2-31号",
			"tel": "02162350365",
			"lat": 31.205960000000001,
			"lng": 121.46859000000001,
			"redeem_time": "10:00-21:30",
			"zone_id": 21,
			"pin_enabled": true,
			"ad_title": "满50减10元",
			"ad_url": "https:\/\/www.baidu.com"
		}],
		"bargain_count": "0",
		"ad": {
			"title": "满50减10元",
			"url": "https:\/\/www.baidu.com"
		},
		"stock_status": "正在抢购中，只剩990个啦",
		"comments_count": "7",
		"rating": 3.2999999999999998,
		"comments_limit": "5",
		"special_tips": {
			"redeem_period": "有效期7天",
			"need_book": "需预约",
			"refund_type": "支持退款"
		},
		"special_tips_array": [{
			"key": "redeem_period",
			"text": "有效期7天",
			"icon": "http:\/\/img2.haoshiqi.net\/ma42ba6cf1950ab6dc9d1031f85414c3e3.jpg"
		}, {
			"key": "need_book",
			"text": "需预约",
			"icon": "http:\/\/img2.haoshiqi.net\/ma5cf32cfc07133ee6c2ada706ffb5afcb.jpg"
		}, {
			"key": "refund_type",
			"text": "支持退款",
			"icon": "http:\/\/img2.haoshiqi.net\/mafd78d0a1d7929b2009ec9757a2e77174.jpg"
		}],
		"tips_array": ["兑换时间：10:00-21:30", "每人每1天每店限购一次", "周末和法定节假日不可用"],
		"is_on_shelf": true,
		"share_info": {
			"title": "原价¥2，最低砍至¥1！绿豆汤"
		},
		"reset_at": 34428,
		"vendor_coupon": 0,
		"cb_types": [1],
		"step_price_first": 1.6000000000000001,
		"bargain_range_second": 0.20000000000000001,
		"step_price_second": 0,
		"bargain_range_third": 0
	}
}
    "#
    .to_string();
    {
        let mut chars = data.chars();
        let mut pit = ParserIter::new(&mut chars);
        println!("{:?}", pit.parse());
    }
}
