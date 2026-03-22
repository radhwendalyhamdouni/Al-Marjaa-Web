// ═══════════════════════════════════════════════════════════════════════════════
// أدوات مساعدة - Utilities
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// تحليل الاستعلام
// ═══════════════════════════════════════════════════════════════════════════════

/// تحليل نص الاستعلام إلى HashMap
pub fn تحليل_الاستعلام(النص: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    for pair in النص.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = فك_ترميز_النص(key);
            let value = فك_ترميز_النص(value);
            result.insert(key, value);
        } else if !pair.is_empty() {
            result.insert(pair.to_string(), String::new());
        }
    }
    
    result
}

/// بناء نص الاستعلام من HashMap
pub fn بناء_الاستعلام(المعاملات: &HashMap<String, String>) -> String {
    المعاملات.iter()
        .map(|(k, v)| format!("{}={}", ترميز_النص(k), ترميز_النص(v)))
        .collect::<Vec<_>>()
        .join("&")
}

// ═══════════════════════════════════════════════════════════════════════════════
// ترميز URL
// ═══════════════════════════════════════════════════════════════════════════════

/// ترميز النص للاستخدام في URL
pub fn ترميز_النص(النص: &str) -> String {
    percent_encoding::percent_encode(النص.as_bytes(), percent_encoding::NON_ALPHANUMERIC)
        .to_string()
}

/// فك ترميز النص من URL
pub fn فك_ترميز_النص(النص: &str) -> String {
    percent_encoding::percent_decode_str(النص)
        .decode_utf8_lossy()
        .to_string()
}

// ═══════════════════════════════════════════════════════════════════════════════
// MIME Types
// ═══════════════════════════════════════════════════════════════════════════════

/// الحصول على MIME Type من امتداد الملف
pub fn نوع_ملف(الامتداد: &str) -> &'static str {
    match الامتداد.to_lowercase().as_str() {
        // نصوص
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "xml" => "application/xml; charset=utf-8",
        "txt" => "text/plain; charset=utf-8",
        "md" => "text/markdown; charset=utf-8",
        
        // صور
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "webp" => "image/webp",
        
        // خطوط
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        
        // صوت
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        
        // فيديو
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        
        // مستندات
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        
        // ضغط
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        
        _ => "application/octet-stream",
    }
}

/// الحصول على MIME Type من اسم الملف
pub fn نوع_من_اسم(الاسم: &str) -> &'static str {
    if let Some(ext) = الاسم.rsplit('.').next() {
        نوع_ملف(ext)
    } else {
        "application/octet-stream"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التحقق من IP
// ═══════════════════════════════════════════════════════════════════════════════

/// التحقق من أن IP محلي
pub fn ip_محلي(ip: &str) -> bool {
    ip.starts_with("127.") ||
    ip.starts_with("192.168.") ||
    ip.starts_with("10.") ||
    ip.starts_with("172.16.") ||
    ip.starts_with("172.17.") ||
    ip.starts_with("172.18.") ||
    ip.starts_with("172.19.") ||
    ip.starts_with("172.20.") ||
    ip.starts_with("172.21.") ||
    ip.starts_with("172.22.") ||
    ip.starts_with("172.23.") ||
    ip.starts_with("172.24.") ||
    ip.starts_with("172.25.") ||
    ip.starts_with("172.26.") ||
    ip.starts_with("172.27.") ||
    ip.starts_with("172.28.") ||
    ip.starts_with("172.29.") ||
    ip.starts_with("172.30.") ||
    ip.starts_with("172.31.") ||
    ip == "::1"
}

// ═══════════════════════════════════════════════════════════════════════════════
// التوليد
// ═══════════════════════════════════════════════════════════════════════════════

/// توليد معرف فريد
pub fn معرف_فريد() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// توليد معرف قصير
pub fn معرف_قصير(الطول: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    (0..الطول)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// توليد رمز تحقق
pub fn رمز_تحقق(الطول: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    (0..الطول)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// الوقت والتاريخ
// ═══════════════════════════════════════════════════════════════════════════════

/// الوقت الحالي بتوقيت UTC
pub fn الوقت_الحالي() -> i64 {
    chrono::Utc::now().timestamp()
}

/// الوقت الحالي بتوقيت محلي
pub fn الوقت_المحلي() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// تنسيق الوقت
pub fn تنسيق_الوقت(الطابع: i64) -> String {
    chrono::DateTime::from_timestamp(الطابع, 0)
        .map(|dt: chrono::DateTime<chrono::Utc>| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Hashing
// ═══════════════════════════════════════════════════════════════════════════════

/// حساب SHA256
pub fn sha256(النص: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(النص.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// حساب MD5 (غير آمن للاستخدام الأمني)
pub fn md5(النص: &str) -> String {
    let digest = md5::compute(النص.as_bytes());
    format!("{:x}", digest)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Base64
// ═══════════════════════════════════════════════════════════════════════════════

/// ترميز Base64
pub fn base64_ترميز(البيانات: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, البيانات)
}

/// فك ترميز Base64
pub fn base64_فك(النص: &str) -> Option<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, النص).ok()
}

/// ترميز Base64 URL
pub fn base64_url_ترميز(البيانات: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, البيانات)
}

/// فك ترميز Base64 URL
pub fn base64_url_فك(النص: &str) -> Option<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, النص).ok()
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod اختبارات {
    use super::*;
    
    #[test]
    fn اختبار_تحليل_الاستعلام() {
        let query = "اسم=أحمد&عمر=25";
        let params = تحليل_الاستعلام(query);
        
        assert_eq!(params.get("اسم").unwrap(), "أحمد");
        assert_eq!(params.get("عمر").unwrap(), "25");
    }
    
    #[test]
    fn اختبار_نوع_ملف() {
        assert_eq!(نوع_ملف("html"), "text/html; charset=utf-8");
        assert_eq!(نوع_ملف("json"), "application/json; charset=utf-8");
        assert_eq!(نوع_ملف("png"), "image/png");
    }
    
    #[test]
    fn اختبار_ip_محلي() {
        assert!(ip_محلي("127.0.0.1"));
        assert!(ip_محلي("192.168.1.1"));
        assert!(!ip_محلي("8.8.8.8"));
    }
    
    #[test]
    fn اختبار_معرف_فريد() {
        let id1 = معرف_فريد();
        let id2 = معرف_فريد();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn اختبار_رمز_تحقق() {
        let code = رمز_تحقق(6);
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_numeric()));
    }
}
