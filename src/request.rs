// ═══════════════════════════════════════════════════════════════════════════════
// الطلب - Request
// ═══════════════════════════════════════════════════════════════════════════════

use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::net::SocketAddr;

// ═══════════════════════════════════════════════════════════════════════════════
// طريقة الطلب
// ═══════════════════════════════════════════════════════════════════════════════

/// طريقة HTTP مع أسماء عربية
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum طريقة_الطلب {
    /// GET - جلب
    جلب,
    /// POST - إرسال
    إرسال,
    /// PUT - تحديث
    تحديث,
    /// DELETE - حذف
    حذف,
    /// PATCH - تعديل جزئي
    تعديل,
    /// HEAD - رأس فقط
    رأس,
    /// OPTIONS - خيارات
    خيارات,
    /// CONNECT - اتصال
    اتصال,
    /// TRACE - تتبع
    تتبع,
}

impl طريقة_الطلب {
    /// تحويل من نص
    pub fn من_النص(النص: &str) -> Option<Self> {
        match النص.to_uppercase().as_str() {
            "GET" => Some(طريقة_الطلب::جلب),
            "POST" => Some(طريقة_الطلب::إرسال),
            "PUT" => Some(طريقة_الطلب::تحديث),
            "DELETE" => Some(طريقة_الطلب::حذف),
            "PATCH" => Some(طريقة_الطلب::تعديل),
            "HEAD" => Some(طريقة_الطلب::رأس),
            "OPTIONS" => Some(طريقة_الطلب::خيارات),
            "CONNECT" => Some(طريقة_الطلب::اتصال),
            "TRACE" => Some(طريقة_الطلب::تتبع),
            _ => None,
        }
    }
    
    /// تحويل إلى نص
    pub fn إلى_النص(&self) -> &'static str {
        match self {
            طريقة_الطلب::جلب => "GET",
            طريقة_الطلب::إرسال => "POST",
            طريقة_الطلب::تحديث => "PUT",
            طريقة_الطلب::حذف => "DELETE",
            طريقة_الطلب::تعديل => "PATCH",
            طريقة_الطلب::رأس => "HEAD",
            طريقة_الطلب::خيارات => "OPTIONS",
            طريقة_الطلب::اتصال => "CONNECT",
            طريقة_الطلب::تتبع => "TRACE",
        }
    }
}

impl std::fmt::Display for طريقة_الطلب {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.إلى_النص())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// رأس الطلب
// ═══════════════════════════════════════════════════════════════════════════════

/// رأس الطلب
#[derive(Debug, Clone)]
pub struct رأس_الطلب {
    pub الاسم: String,
    pub القيمة: String,
}

impl رأس_الطلب {
    pub fn جديد<S: Into<String>>(الاسم: S, القيمة: S) -> Self {
        رأس_الطلب {
            الاسم: الاسم.into(),
            القيمة: القيمة.into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الطلب
// ═══════════════════════════════════════════════════════════════════════════════

/// طلب HTTP
#[derive(Debug, Clone)]
pub struct طلب {
    /// الطريقة (GET, POST, ...)
    pub الطريقة: طريقة_الطلب,
    
    /// المسار (URL)
    pub المسار: String,
    
    /// معاملات المسار (مثل {معرف})
    pub المعاملات: HashMap<String, String>,
    
    /// معاملات الاستعلام (Query)
    pub الاستعلام: HashMap<String, String>,
    
    /// الرؤوس
    pub الرؤوس: HashMap<String, String>,
    
    /// محتوى الطلب (Body)
    pub المحتوى: Vec<u8>,
    
    /// عنوان العميل
    pub عنوان_العميل: Option<SocketAddr>,
    
    /// Cookies
    pub الكوكيز: HashMap<String, String>,
}

impl طلب {
    /// إنشاء طلب جديد
    pub fn جديد() -> Self {
        طلب {
            الطريقة: طريقة_الطلب::جلب,
            المسار: "/".to_string(),
            المعاملات: HashMap::new(),
            الاستعلام: HashMap::new(),
            الرؤوس: HashMap::new(),
            المحتوى: Vec::new(),
            عنوان_العميل: None,
            الكوكيز: HashMap::new(),
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // معلومات أساسية
    // ─────────────────────────────────────────────────────────────────────
    
    /// هل الطلب GET؟
    pub fn جلب(&self) -> bool {
        self.الطريقة == طريقة_الطلب::جلب
    }
    
    /// هل الطلب POST؟
    pub fn إرسال(&self) -> bool {
        self.الطريقة == طريقة_الطلب::إرسال
    }
    
    /// هل الطلب PUT؟
    pub fn تحديث(&self) -> bool {
        self.الطريقة == طريقة_الطلب::تحديث
    }
    
    /// هل الطلب DELETE؟
    pub fn حذف(&self) -> bool {
        self.الطريقة == طريقة_الطلب::حذف
    }
    
    /// الحصول على المسار بدون استعلام
    pub fn المسار_النظيف(&self) -> &str {
        if let Some(pos) = self.المسار.find('?') {
            &self.المسار[..pos]
        } else {
            &self.المسار
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // المعاملات
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على معامل من المسار
    pub fn معامل(&self, الاسم: &str) -> Option<&String> {
        self.المعاملات.get(الاسم)
    }
    
    /// الحصول على معامل أو قيمة افتراضية
    pub fn معامل_أو(&self, الاسم: &str, الافتراضي: &str) -> String {
        self.معامل(الاسم)
            .map(|s| s.clone())
            .unwrap_or_else(|| الافتراضي.to_string())
    }
    
    /// الحصول على معامل استعلام
    pub fn استعلام(&self, الاسم: &str) -> Option<&String> {
        self.الاستعلام.get(الاسم)
    }
    
    /// الحصول على استعلام أو قيمة افتراضية
    pub fn استعلام_أو(&self, الاسم: &str, الافتراضي: &str) -> String {
        self.استعلام(الاسم)
            .map(|s| s.clone())
            .unwrap_or_else(|| الافتراضي.to_string())
    }
    
    /// الحصول على جميع معاملات الاستعلام
    pub fn جميع_الاستعلامات(&self) -> &HashMap<String, String> {
        &self.الاستعلام
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // الرؤوس
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على رأس
    pub fn رأس(&self, الاسم: &str) -> Option<&String> {
        // البحث بدون حساسية لحالة الأحرف
        self.الرؤوس.iter()
            .find(|(k, _)| k.to_lowercase() == الاسم.to_lowercase())
            .map(|(_, v)| v)
    }
    
    /// الحصول على Content-Type
    pub fn نوع_المحتوى(&self) -> Option<&String> {
        self.رأس("Content-Type")
    }
    
    /// الحصول على User-Agent
    pub fn وكيل_المستخدم(&self) -> Option<&String> {
        self.رأس("User-Agent")
    }
    
    /// الحصول على Authorization
    pub fn التفويض(&self) -> Option<&String> {
        self.رأس("Authorization")
    }
    
    /// الحصول على Bearer Token
    pub fn رمز_الحامل(&self) -> Option<String> {
        self.التفويض()
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| h[7..].to_string())
    }
    
    /// الحصول على Host
    pub fn المضيف(&self) -> Option<&String> {
        self.رأس("Host")
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // المحتوى
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على المحتوى كنص
    pub fn المحتوى_كنص(&self) -> String {
        String::from_utf8_lossy(&self.المحتوى).to_string()
    }
    
    /// الحصول على المحتوى كـ JSON
    pub fn المحتوى_كjson<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.المحتوى)
    }
    
    /// الحصول على المحتوى كـ JSON غير مُحلل
    pub fn المحتوى_كjson_خام(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_slice(&self.المحتوى)
    }
    
    /// الحصول على محتوى النموذج (form-data)
    pub fn المحتوى_كنموذج(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let content = self.المحتوى_كنص();
        
        for pair in content.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = urlencoding_decode(key);
                let value = urlencoding_decode(value);
                result.insert(key, value);
            }
        }
        
        result
    }
    
    /// حجم المحتوى
    pub fn حجم_المحتوى(&self) -> usize {
        self.المحتوى.len()
    }
    
    /// هل المحتوى فارغ
    pub fn محتوى_فارغ(&self) -> bool {
        self.المحتوى.is_empty()
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // Cookies
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على Cookie
    pub fn كوكي(&self, الاسم: &str) -> Option<&String> {
        self.الكوكيز.get(الاسم)
    }
    
    /// الحصول على Cookie أو قيمة افتراضية
    pub fn كوكي_أو(&self, الاسم: &str, الافتراضي: &str) -> String {
        self.كوكي(الاسم)
            .cloned()
            .unwrap_or_else(|| الافتراضي.to_string())
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // معلومات العميل
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على IP العميل
    pub fn ip_العميل(&self) -> Option<std::net::IpAddr> {
        self.عنوان_العميل.map(|a| a.ip())
    }
    
    /// الحصول على منفذ العميل
    pub fn منفذ_العميل(&self) -> Option<u16> {
        self.عنوان_العميل.map(|a| a.port())
    }
    
    /// هل الطلب من localhost
    pub fn محلي(&self) -> bool {
        self.ip_العميل()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false)
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // إنشاء ردود
    // ─────────────────────────────────────────────────────────────────────
    
    /// إنشاء رد نصي
    pub fn رد_بنص<S: Into<String>>(&self, المحتوى: S) -> crate::response::رد {
        crate::response::رد::نص(المحتوى)
    }
    
    /// إنشاء رد JSON
    pub fn رد_بjson<T: serde::Serialize>(&self, البيانات: T) -> crate::response::رد {
        crate::response::رد::json(البيانات)
    }
    
    /// إنشاء رد HTML
    pub fn رد_بhtml<S: Into<String>>(&self, المحتوى: S) -> crate::response::رد {
        crate::response::رد::html(المحتوى)
    }
    
    /// إنشاء رد خطأ
    pub fn رد_بخطأ<S: Into<String>>(&self, الرسالة: S, الكود: u16) -> crate::response::رد {
        crate::response::رد::خطأ(الرسالة, الكود)
    }
}

impl Default for طلب {
    fn default() -> Self {
        طلب::جديد()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// فك تشفير URL
fn urlencoding_decode(input: &str) -> String {
    percent_encoding::percent_decode_str(input)
        .decode_utf8_lossy()
        .to_string()
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod اختبارات {
    use super::*;
    
    #[test]
    fn اختبار_طريقة_الطلب() {
        assert_eq!(طريقة_الطلب::من_النص("GET"), Some(طريقة_الطلب::جلب));
        assert_eq!(طريقة_الطلب::من_النص("POST"), Some(طريقة_الطلب::إرسال));
    }
    
    #[test]
    fn اختبار_طلب_جديد() {
        let request = طلب::جديد();
        assert_eq!(request.الطريقة, طريقة_الطلب::جلب);
        assert_eq!(request.المسار, "/");
    }
}
