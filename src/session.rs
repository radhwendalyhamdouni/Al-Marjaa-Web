// ═══════════════════════════════════════════════════════════════════════════════
// الجلسات - Sessions
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════════
// الجلسة
// ═══════════════════════════════════════════════════════════════════════════════

/// جلسة المستخدم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct جلسة {
    /// معرف الجلسة
    pub المعرف: String,
    /// البيانات
    pub البيانات: HashMap<String, serde_json::Value>,
    /// وقت الإنشاء
    pub وقت_الإنشاء: i64,
    /// آخر نشاط
    pub آخر_نشاط: i64,
    /// وقت الانتهاء (بالثواني)
    pub مدة_الصلاحية: i64,
}

impl جلسة {
    /// إنشاء جلسة جديدة
    pub fn جديدة() -> Self {
        let now = chrono::Utc::now().timestamp();
        
        جلسة {
            المعرف: uuid::Uuid::new_v4().to_string(),
            البيانات: HashMap::new(),
            وقت_الإنشاء: now,
            آخر_نشاط: now,
            مدة_الصلاحية: 3600, // ساعة افتراضية
        }
    }
    
    /// إنشاء جلسة مع مدة صلاحية
    pub fn مع_مدة(مدة_بالثواني: i64) -> Self {
        let mut session = جلسة::جديدة();
        session.مدة_الصلاحية = مدة_بالثواني;
        session
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // إدارة البيانات
    // ─────────────────────────────────────────────────────────────────────
    
    /// الحصول على قيمة
    pub fn احصل<T: for<'de> Deserialize<'de>>(&self, المفتاح: &str) -> Option<T> {
        self.البيانات.get(المفتاح)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    /// الحصول على نص
    pub fn احصل_نص(&self, المفتاح: &str) -> Option<String> {
        self.البيانات.get(المفتاح)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }
    
    /// الحصول على رقم
    pub fn احصل_رقم(&self, المفتاح: &str) -> Option<i64> {
        self.البيانات.get(المفتاح)
            .and_then(|v| v.as_i64())
    }
    
    /// الحصول على منطقي
    pub fn احصل_منطقي(&self, المفتاح: &str) -> Option<bool> {
        self.البيانات.get(المفتاح)
            .and_then(|v| v.as_bool())
    }
    
    /// تعيين قيمة
    pub fn عيّن<T: Serialize>(&mut self, المفتاح: &str, القيمة: T) {
        if let Ok(value) = serde_json::to_value(القيمة) {
            self.البيانات.insert(المفتاح.to_string(), value);
        }
        self.تحديث_النشاط();
    }
    
    /// تعيين نص
    pub fn عيّن_نص(&mut self, المفتاح: &str, القيمة: &str) {
        self.عيّن(المفتاح, القيمة);
    }
    
    /// حذف قيمة
    pub fn احذف(&mut self, المفتاح: &str) {
        self.البيانات.remove(المفتاح);
        self.تحديث_النشاط();
    }
    
    /// التحقق من وجود مفتاح
    pub fn يحتوي(&self, المفتاح: &str) -> bool {
        self.البيانات.contains_key(المفتاح)
    }
    
    /// مسح جميع البيانات
    pub fn امسح(&mut self) {
        self.البيانات.clear();
        self.تحديث_النشاط();
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // حالة الجلسة
    // ─────────────────────────────────────────────────────────────────────
    
    /// هل الجلسة منتهية
    pub fn منتهية(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        (now - self.آخر_نشاط) > self.مدة_الصلاحية
    }
    
    /// تحديث النشاط
    pub fn تحديث_النشاط(&mut self) {
        self.آخر_نشاط = chrono::Utc::now().timestamp();
    }
    
    /// الوقت المتبقي
    pub fn الوقت_المتبقي(&self) -> i64 {
        let now = chrono::Utc::now().timestamp();
        let elapsed = now - self.آخر_نشاط;
        self.مدة_الصلاحية - elapsed
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // بيانات المستخدم
    // ─────────────────────────────────────────────────────────────────────
    
    /// تسجيل دخول مستخدم
    pub fn دخول(&mut self, معرف_المستخدم: &str, الاسم: &str) {
        self.عيّن("مستخدم_مسجل", true);
        self.عيّن("معرف_المستخدم", معرف_المستخدم);
        self.عيّن("اسم_المستخدم", الاسم);
    }
    
    /// تسجيل خروج
    pub fn خروج(&mut self) {
        self.احذف("مستخدم_مسجل");
        self.احذف("معرف_المستخدم");
        self.احذف("اسم_المستخدم");
    }
    
    /// هل المستخدم مسجل
    pub fn مسجل(&self) -> bool {
        self.احصل_منطقي("مستخدم_مسجل").unwrap_or(false)
    }
    
    /// معرف المستخدم
    pub fn معرف_المستخدم(&self) -> Option<String> {
        self.احصل_نص("معرف_المستخدم")
    }
    
    /// اسم المستخدم
    pub fn اسم_المستخدم(&self) -> Option<String> {
        self.احصل_نص("اسم_المستخدم")
    }
}

impl Default for جلسة {
    fn default() -> Self {
        جلسة::جديدة()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مخزن الجلسات
// ═══════════════════════════════════════════════════════════════════════════════

/// مخزن الجلسات
pub struct مخزن_الجلسات {
    /// الجلسات
    sessions: RwLock<HashMap<String, جلسة>>,
    /// مدة الصلاحية الافتراضية
    default_ttl: i64,
}

impl مخزن_الجلسات {
    /// إنشاء مخزن جديد
    pub fn جديد() -> Self {
        مخزن_الجلسات {
            sessions: RwLock::new(HashMap::new()),
            default_ttl: 3600,
        }
    }
    
    /// إنشاء مخزن مع مدة صلاحية
    pub fn مع_مدة(مدة_بالثواني: i64) -> Self {
        مخزن_الجلسات {
            sessions: RwLock::new(HashMap::new()),
            default_ttl: مدة_بالثواني,
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // إدارة الجلسات
    // ─────────────────────────────────────────────────────────────────────
    
    /// إنشاء جلسة جديدة
    pub fn أنشئ(&self) -> جلسة {
        let session = جلسة::مع_مدة(self.default_ttl);
        self.sessions.write().insert(session.المعرف.clone(), session.clone());
        session
    }
    
    /// الحصول على جلسة
    pub fn احصل(&self, المعرف: &str) -> Option<جلسة> {
        let sessions = self.sessions.read();
        sessions.get(المعرف)
            .filter(|s| !s.منتهية())
            .cloned()
    }
    
    /// تحديث جلسة
    pub fn حدث(&self, الجلسة: &جلسة) {
        let mut sessions = self.sessions.write();
        sessions.insert(الجلسة.المعرف.clone(), الجلسة.clone());
    }
    
    /// حذف جلسة
    pub fn احذف(&self, المعرف: &str) {
        self.sessions.write().remove(المعرف);
    }
    
    /// عدد الجلسات
    pub fn عدد(&self) -> usize {
        self.sessions.read().len()
    }
    
    /// عدد الجلسات النشطة
    pub fn عدد_النشطة(&self) -> usize {
        self.sessions.read().values()
            .filter(|s| !s.منتهية())
            .count()
    }
    
    /// تنظيف الجلسات المنتهية
    pub fn نظف(&self) -> usize {
        let mut sessions = self.sessions.write();
        let before = sessions.len();
        sessions.retain(|_, s| !s.منتهية());
        before - sessions.len()
    }
}

impl Default for مخزن_الجلسات {
    fn default() -> Self {
        مخزن_الجلسات::جديد()
    }
}
