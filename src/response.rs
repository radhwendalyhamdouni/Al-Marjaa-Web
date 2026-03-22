// ═══════════════════════════════════════════════════════════════════════════════
// الرد - Response
// ═══════════════════════════════════════════════════════════════════════════════

use bytes::Bytes;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════════
// حالة HTTP
// ═══════════════════════════════════════════════════════════════════════════════

/// حالة HTTP مع أسماء عربية
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct حالة {
    pub الكود: u16,
    pub النص: &'static str,
}

impl حالة {
    // ─────────────────────────────────────────────────────────────────────
    // حالات النجاح (2xx)
    // ─────────────────────────────────────────────────────────────────────
    
    /// 200 - نجاح
    pub const نجاح: حالة = حالة { الكود: 200, النص: "OK" };
    
    /// 201 - تم الإنشاء
    pub const تم_الإنشاء: حالة = حالة { الكود: 201, النص: "Created" };
    
    /// 202 - مقبول
    pub const مقبول: حالة = حالة { الكود: 202, النص: "Accepted" };
    
    /// 204 - بدون محتوى
    pub const بدون_محتوى: حالة = حالة { الكود: 204, النص: "No Content" };
    
    // ─────────────────────────────────────────────────────────────────────
    // حالات إعادة التوجيه (3xx)
    // ─────────────────────────────────────────────────────────────────────
    
    /// 301 - نقل دائم
    pub const نقل_دائم: حالة = حالة { الكود: 301, النص: "Moved Permanently" };
    
    /// 302 - نقل مؤقت
    pub const نقل_مؤقت: حالة = حالة { الكود: 302, النص: "Found" };
    
    /// 304 - غير معدل
    pub const غير_معدل: حالة = حالة { الكود: 304, النص: "Not Modified" };
    
    /// 307 - إعادة توجيه مؤقتة
    pub const توجيه_مؤقت: حالة = حالة { الكود: 307, النص: "Temporary Redirect" };
    
    /// 308 - إعادة توجيه دائمة
    pub const توجيه_دائم: حالة = حالة { الكود: 308, النص: "Permanent Redirect" };
    
    // ─────────────────────────────────────────────────────────────────────
    // حالات خطأ العميل (4xx)
    // ─────────────────────────────────────────────────────────────────────
    
    /// 400 - طلب سيئ
    pub const طلب_سيئ: حالة = حالة { الكود: 400, النص: "Bad Request" };
    
    /// 401 - غير مصرح
    pub const غير_مصرح: حالة = حالة { الكود: 401, النص: "Unauthorized" };
    
    /// 403 - محظور
    pub const محظور: حالة = حالة { الكود: 403, النص: "Forbidden" };
    
    /// 404 - غير موجود
    pub const غير_موجود: حالة = حالة { الكود: 404, النص: "Not Found" };
    
    /// 405 - طريقة غير مسموحة
    pub const طريقة_غير_مسموحة: حالة = حالة { الكود: 405, النص: "Method Not Allowed" };
    
    /// 408 - انتهت المهلة
    pub const انتهت_المهلة: حالة = حالة { الكود: 408, النص: "Request Timeout" };
    
    /// 409 - تضارب
    pub const تضارب: حالة = حالة { الكود: 409, النص: "Conflict" };
    
    /// 410 - محذوف
    pub const محذوف: حالة = حالة { الكود: 410, النص: "Gone" };
    
    /// 413 - المحتوى كبير جداً
    pub const محتوى_كبير: حالة = حالة { الكود: 413, النص: "Payload Too Large" };
    
    /// 422 - كيان غير قابل للمعالجة
    pub const غير_قابل_للمعالجة: حالة = حالة { الكود: 422, النص: "Unprocessable Entity" };
    
    /// 429 - طلبات كثيرة جداً
    pub const طلبات_كثيرة: حالة = حالة { الكود: 429, النص: "Too Many Requests" };
    
    // ─────────────────────────────────────────────────────────────────────
    // حالات خطأ الخادم (5xx)
    // ─────────────────────────────────────────────────────────────────────
    
    /// 500 - خطأ داخلي
    pub const خطأ_داخلي: حالة = حالة { الكود: 500, النص: "Internal Server Error" };
    
    /// 501 - غير منفذ
    pub const غير_منفذ: حالة = حالة { الكود: 501, النص: "Not Implemented" };
    
    /// 502 - بوابة سيئة
    pub const بوابة_سيئة: حالة = حالة { الكود: 502, النص: "Bad Gateway" };
    
    /// 503 - الخدمة غير متاحة
    pub const خدمة_غير_متاحة: حالة = حالة { الكود: 503, النص: "Service Unavailable" };
    
    /// 504 - انتهت مهلة البوابة
    pub const مهلة_البوابة: حالة = حالة { الكود: 504, النص: "Gateway Timeout" }
}

impl Default for حالة {
    fn default() -> Self {
        حالة::نجاح
    }
}

impl fmt::Display for حالة {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.الكود, self.النص)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// رأس الرد
// ═══════════════════════════════════════════════════════════════════════════════

/// رأس الرد
#[derive(Debug, Clone)]
pub struct رأس_الرد {
    pub الاسم: String,
    pub القيمة: String,
}

impl رأس_الرد {
    /// إنشاء رأس جديد
    pub fn جديد<S: Into<String>>(الاسم: S, القيمة: S) -> Self {
        رأس_الرد {
            الاسم: الاسم.into(),
            القيمة: القيمة.into(),
        }
    }
    
    /// Content-Type
    pub fn نوع_المحتوى(القيمة: &str) -> Self {
        رأس_الرد::جديد("Content-Type", القيمة)
    }
    
    /// JSON
    pub fn json() -> Self {
        رأس_الرد::نوع_المحتوى("application/json; charset=utf-8")
    }
    
    /// HTML
    pub fn html() -> Self {
        رأس_الرد::نوع_المحتوى("text/html; charset=utf-8")
    }
    
    /// نص عادي
    pub fn نص() -> Self {
        رأس_الرد::نوع_المحتوى("text/plain; charset=utf-8")
    }
    
    /// CSS
    pub fn css() -> Self {
        رأس_الرد::نوع_المحتوى("text/css; charset=utf-8")
    }
    
    /// JavaScript
    pub fn javascript() -> Self {
        رأس_الرد::نوع_المحتوى("application/javascript; charset=utf-8")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الرد
// ═══════════════════════════════════════════════════════════════════════════════

/// الرد على الطلب
#[derive(Debug)]
pub struct رد {
    /// حالة HTTP
    pub حالة: حالة,
    /// الرؤوس
    pub رؤوس: HashMap<String, String>,
    /// المحتوى
    pub المحتوى: Vec<u8>,
    /// نوع المحتوى
    pub نوع_المحتوى: String,
}

impl Default for رد {
    fn default() -> Self {
        رد {
            حالة: حالة::نجاح,
            رؤوس: HashMap::new(),
            المحتوى: Vec::new(),
            نوع_المحتوى: "text/plain; charset=utf-8".to_string(),
        }
    }
}

impl رد {
    /// إنشاء رد جديد
    pub fn جديد() -> Self {
        رد::default()
    }
    
    /// إنشاء رد بحالة
    pub fn بحالة(حالة: حالة) -> Self {
        رد {
            حالة,
            ..رد::default()
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // أنواع الردود
    // ─────────────────────────────────────────────────────────────────────
    
    /// رد نصي
    pub fn نص<S: Into<String>>(المحتوى: S) -> Self {
        let text = المحتوى.into();
        رد {
            حالة: حالة::نجاح,
            المحتوى: text.into_bytes(),
            نوع_المحتوى: "text/plain; charset=utf-8".to_string(),
            ..رد::default()
        }
    }
    
    /// رد HTML
    pub fn html<S: Into<String>>(المحتوى: S) -> Self {
        let text = المحتوى.into();
        رد {
            حالة: حالة::نجاح,
            المحتوى: text.into_bytes(),
            نوع_المحتوى: "text/html; charset=utf-8".to_string(),
            ..رد::default()
        }
    }
    
    /// رد JSON
    pub fn json<T: Serialize>(البيانات: T) -> Self {
        let text = serde_json::to_string_pretty(&البيانات).unwrap_or_else(|_| "{}".to_string());
        رد {
            حالة: حالة::نجاح,
            المحتوى: text.into_bytes(),
            نوع_المحتوى: "application/json; charset=utf-8".to_string(),
            ..رد::default()
        }
    }
    
    /// رد ناجح (JSON مع حالة نجاح)
    pub fn نجاح<T: Serialize>(البيانات: T) -> Self {
        let response = serde_json::json!({
            "نجاح": true,
            "البيانات": البيانات
        });
        رد::json(response)
    }
    
    /// رد بخطأ
    pub fn خطأ<S: Into<String>>(الرسالة: S, الكود: u16) -> Self {
        let status = match الكود {
            400 => حالة::طلب_سيئ,
            401 => حالة::غير_مصرح,
            403 => حالة::محظور,
            404 => حالة::غير_موجود,
            422 => حالة::غير_قابل_للمعالجة,
            429 => حالة::طلبات_كثيرة,
            500 => حالة::خطأ_داخلي,
            503 => حالة::خدمة_غير_متاحة,
            _ => حالة { الكود, النص: "Error" },
        };
        
        let response = serde_json::json!({
            "نجاح": false,
            "خطأ": الرسالة.into(),
            "الكود": الكود
        });
        
        رد {
            حالة: status,
            المحتوى: serde_json::to_string_pretty(&response).unwrap().into_bytes(),
            نوع_المحتوى: "application/json; charset=utf-8".to_string(),
            ..رد::default()
        }
    }
    
    /// إعادة توجيه
    pub fn توجه<S: Into<String>>(العنوان: S) -> Self {
        let mut response = رد::بحالة(حالة::توجيه_مؤقت);
        response.رؤوس.insert("Location".to_string(), العنوان.into());
        response
    }
    
    /// رد فارغ
    pub fn فارغ() -> Self {
        رد::بحالة(حالة::بدون_محتوى)
    }
    
    /// صفحة غير موجودة (404)
    pub fn غير_موجود<S: Into<String>>(الرسالة: S) -> Self {
        رد::خطأ(الرسالة, 404)
    }
    
    /// غير مصرح (401)
    pub fn غير_مصرح<S: Into<String>>(الرسالة: S) -> Self {
        رد::خطأ(الرسالة, 401)
    }
    
    /// محظور (403)
    pub fn محظور<S: Into<String>>(الرسالة: S) -> Self {
        رد::خطأ(الرسالة, 403)
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // تعديل الرد
    // ─────────────────────────────────────────────────────────────────────
    
    /// تغيير الحالة
    pub fn بحالة_جديدة(mut self, حالة: حالة) -> Self {
        self.حالة = حالة;
        self
    }
    
    /// إضافة رأس
    pub fn أضف_رأس<S: Into<String>>(mut self, الاسم: S, القيمة: S) -> Self {
        self.رؤوس.insert(الاسم.into(), القيمة.into());
        self
    }
    
    /// إضافة رؤوس متعددة
    pub fn أضف_رؤوس(mut self, رؤوس: HashMap<String, String>) -> Self {
        self.رؤوس.extend(رؤوس);
        self
    }
    
    /// تغيير نوع المحتوى
    pub fn بنوع(mut self, النوع: &str) -> Self {
        self.نوع_المحتوى = النوع.to_string();
        self
    }
    
    /// إضافة Cookie
    pub fn أضف_كوكي(mut self, الاسم: &str, القيمة: &str) -> Self {
        let cookie = format!("{}={}; Path=/; HttpOnly; SameSite=Strict", الاسم, القيمة);
        self.رؤوس.insert("Set-Cookie".to_string(), cookie);
        self
    }
    
    /// إضافة Cookie آمن
    pub fn أضف_كوكي_آمن(mut self, الاسم: &str, القيمة: &str) -> Self {
        let cookie = format!("{}={}; Path=/; HttpOnly; Secure; SameSite=Strict", الاسم, القيمة);
        self.رؤوس.insert("Set-Cookie".to_string(), cookie);
        self
    }
    
    /// تفعيل CORS
    pub fn مع_cors(mut self) -> Self {
        self.رؤوس.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        self.رؤوس.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
        self.رؤوس.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
        self
    }
    
    /// CORS مع أصل محدد
    pub fn مع_cors_محدد(mut self, الأصل: &str) -> Self {
        self.رؤوس.insert("Access-Control-Allow-Origin".to_string(), الأصل.to_string());
        self.رؤوس.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
        self.رؤوس.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
        self.رؤوس.insert("Access-Control-Allow-Credentials".to_string(), "true".to_string());
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // تحويلات
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على المحتوى كنص
    pub fn كنص(&self) -> String {
        String::from_utf8_lossy(&self.المحتوى).to_string()
    }
    
    /// الحصول على الحجم
    pub fn الحجم(&self) -> usize {
        self.المحتوى.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Trait للتحويل إلى رد
// ═══════════════════════════════════════════════════════════════════════════════

/// سمة للتحويل إلى رد
pub trait إلى_رد {
    fn إلى_رد(self) -> رد;
}

impl إلى_رد for String {
    fn إلى_رد(self) -> رد {
        رد::نص(self)
    }
}

impl إلى_رد for &str {
    fn إلى_رد(self) -> رد {
        رد::نص(self)
    }
}

impl إلى_رد for رد {
    fn إلى_رد(self) -> رد {
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod اختبارات {
    use super::*;
    
    #[test]
    fn اختبار_رد_نصي() {
        let response = رد::نص("مرحباً");
        assert_eq!(response.حالة.الكود, 200);
        assert_eq!(response.كنص(), "مرحباً");
    }
    
    #[test]
    fn اختبار_رد_json() {
        let response = رد::json(serde_json::json!({"رسالة": "مرحباً"}));
        assert_eq!(response.حالة.الكود, 200);
        assert!(response.نوع_المحتوى.contains("json"));
    }
    
    #[test]
    fn اختبار_رد_خطأ() {
        let response = رد::خطأ("غير موجود", 404);
        assert_eq!(response.حالة.الكود, 404);
    }
}
