// ═══════════════════════════════════════════════════════════════════════════════
// WebSocket - الاتصال في الوقت الحقيقي
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════════
// رسالة WebSocket
// ═══════════════════════════════════════════════════════════════════════════════

/// رسالة WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct رسالة_ويب {
    /// نوع الرسالة
    pub النوع: نوع_الرسالة,
    /// المحتوى
    pub المحتوى: String,
    /// المرسل (اختياري)
    pub المرسل: Option<String>,
    /// الوقت
    pub الوقت: i64,
}

/// نوع الرسالة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum نوع_الرسالة {
    /// نص
    نص,
    /// JSON
    json,
    /// ثنائي
    ثنائي,
    /// اتصال
    اتصال,
    /// قطع اتصال
    قطع,
    /// تنبيه
    تنبيه,
    /// خطأ
    خطأ,
}

impl رسالة_ويب {
    /// إنشاء رسالة نصية
    pub fn نص(المحتوى: &str) -> Self {
        رسالة_ويب {
            النوع: نوع_الرسالة::نص,
            المحتوى: المحتوى.to_string(),
            المرسل: None,
            الوقت: chrono::Utc::now().timestamp(),
        }
    }
    
    /// إنشاء رسالة JSON
    pub fn json<T: Serialize>(البيانات: T) -> Self {
        رسالة_ويب {
            النوع: نوع_الرسالة::json,
            المحتوى: serde_json::to_string(&البيانات).unwrap_or_default(),
            المرسل: None,
            الوقت: chrono::Utc::now().timestamp(),
        }
    }
    
    /// إنشاء رسالة اتصال
    pub fn اتصال(المرسل: &str) -> Self {
        رسالة_ويب {
            النوع: نوع_الرسالة::اتصال,
            المحتوى: "متصل".to_string(),
            المرسل: Some(المرسل.to_string()),
            الوقت: chrono::Utc::now().timestamp(),
        }
    }
    
    /// إنشاء رسالة قطع
    pub fn قطع(المرسل: &str) -> Self {
        رسالة_ويب {
            النوع: نوع_الرسالة::قطع,
            المحتوى: "تم قطع الاتصال".to_string(),
            المرسل: Some(المرسل.to_string()),
            الوقت: chrono::Utc::now().timestamp(),
        }
    }
    
    /// تحويل إلى JSON
    pub fn إلى_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    
    /// تحليل من JSON
    pub fn من_json(النص: &str) -> Option<Self> {
        serde_json::from_str(النص).ok()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اتصال WebSocket
// ═══════════════════════════════════════════════════════════════════════════════

/// اتصال WebSocket
pub struct اتصال_ويب {
    /// معرف الاتصال
    pub المعرف: String,
    /// قناة الإرسال
    pub المرسل: broadcast::Sender<رسالة_ويب>,
    /// قناة الاستقبال
    pub المستقبل: broadcast::Receiver<رسالة_ويب>,
}

impl اتصال_ويب {
    /// إنشاء اتصال جديد
    pub fn جديد() -> Self {
        let (sender, receiver) = broadcast::channel(100);
        let id = uuid::Uuid::new_v4().to_string();
        
        اتصال_ويب {
            المعرف: id,
            المرسل: sender,
            المستقبل: receiver,
        }
    }
    
    /// إرسال رسالة
    pub fn أرسل(&self, الرسالة: رسالة_ويب) -> Result<(), String> {
        self.المرسل.send(الرسالة)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
    
    /// استقبال رسالة
    pub async fn استقبل(&mut self) -> Option<رسالة_ويب> {
        self.المستقبل.recv().await.ok()
    }
}

impl Default for اتصال_ويب {
    fn default() -> Self {
        اتصال_ويب::جديد()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// غرفة WebSocket
// ═══════════════════════════════════════════════════════════════════════════════

/// غرفة للمحادثات الجماعية
pub struct غرفة {
    /// اسم الغرفة
    pub الاسم: String,
    /// قناة البث
    pub البث: broadcast::Sender<رسالة_ويب>,
}

impl غرفة {
    /// إنشاء غرفة جديدة
    pub fn جديدة(الاسم: &str) -> Self {
        let (sender, _) = broadcast::channel(1000);
        
        غرفة {
            الاسم: الاسم.to_string(),
            البث: sender,
        }
    }
    
    /// الانضمام للغرفة
    pub fn انضمام(&self) -> broadcast::Receiver<رسالة_ويب> {
        self.البث.subscribe()
    }
    
    /// بث رسالة للجميع
    pub fn بث(&self, الرسالة: رسالة_ويب) -> Result<usize, String> {
        self.البث.send(الرسالة)
            .map_err(|e| e.to_string())
    }
}
