// ═══════════════════════════════════════════════════════════════════════════════
// أخطاء الويب - Web Errors
// ═══════════════════════════════════════════════════════════════════════════════

use std::fmt;
use std::io;
use thiserror::Error;

/// خطأ الويب الرئيسي
#[derive(Error, Debug)]
pub enum خطأ_ويب {
    /// خطأ في الطلب
    #[error("خطأ في الطلب: {0}")]
    خطأ_طلب(String),
    
    /// خطأ في الرد
    #[error("خطأ في الرد: {0}")]
    خطأ_رد(String),
    
    /// خطأ في التوجيه
    #[error("المسار '{0}' غير موجود")]
    مسار_غير_موجود(String),
    
    /// خطأ في JSON
    #[error("خطأ في تحليل JSON: {0}")]
    خطأ_JSON(String),
    
    /// خطأ في IO
    #[error("خطأ في الإدخال/الإخراج: {0}")]
    خطأ_IO(String),
    
    /// خطأ في الخادم
    #[error("خطأ في الخادم: {0}")]
    خطأ_خادم(String),
    
    /// خطأ في WebSocket
    #[error("خطأ في WebSocket: {0}")]
    خطأ_ويب_سوكت(String),
    
    /// خطأ في TLS
    #[error("خطأ في التشفير: {0}")]
    خطأ_TLS(String),
    
    /// خطأ في الجلسة
    #[error("خطأ في الجلسة: {0}")]
    خطأ_جلسة(String),
    
    /// طلب غير مصرح
    #[error("غير مصرح: {0}")]
    غير_مصرح(String),
    
    /// طلب محظور
    #[error("محظور: {0}")]
    محظور(String),
    
    /// الصفحة غير موجودة
    #[error("الصفحة غير موجودة: {0}")]
    غير_موجود(String),
    
    /// خطأ داخلي
    #[error("خطأ داخلي: {0}")]
    خطأ_داخلي(String),
}

impl From<io::Error> for خطأ_ويب {
    fn from(err: io::Error) -> Self {
        خطأ_ويب::خطأ_IO(err.to_string())
    }
}

impl From<serde_json::Error> for خطأ_ويب {
    fn from(err: serde_json::Error) -> Self {
        خطأ_ويب::خطأ_JSON(err.to_string())
    }
}

impl From<hyper::Error> for خطأ_ويب {
    fn from(err: hyper::Error) -> Self {
        خطأ_ويب::خطأ_خادم(err.to_string())
    }
}

/// نتيجة الويب
pub type نتيجة_ويب<T> = Result<T, خطأ_ويب>;
