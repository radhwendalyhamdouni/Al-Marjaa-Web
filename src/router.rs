// ═══════════════════════════════════════════════════════════════════════════════
// الموجّه - Router
// ═══════════════════════════════════════════════════════════════════════════════

use crate::request::طلب;
use crate::response::رد;
use crate::request::طريقة_الطلب;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
// نوع المعالج
// ═══════════════════════════════════════════════════════════════════════════════

/// معالج الطلب
pub type معالج = Arc<dyn Fn(طلب) -> رد + Send + Sync>;

/// معالج غير متزامن
pub type معالج_غير_متزامن = Arc<dyn Fn(طلب) -> std::pin::Pin<Box<dyn std::future::Future<Output = رد> + Send>> + Send + Sync>;

// ═══════════════════════════════════════════════════════════════════════════════
// طريقة التوجيه
// ═══════════════════════════════════════════════════════════════════════════════

/// طريقة HTTP للتوجيه
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum طريقة {
    /// جميع الطرق
    الكل,
    /// GET
    جلب,
    /// POST
    إرسال,
    /// PUT
    تحديث,
    /// DELETE
    حذف,
    /// PATCH
    تعديل,
    /// OPTIONS
    خيارات,
}

impl From<طريقة_الطلب> for طريقة {
    fn from(method: طريقة_الطلب) -> Self {
        match method {
            طريقة_الطلب::جلب => طريقة::جلب,
            طريقة_الطلب::إرسال => طريقة::إرسال,
            طريقة_الطلب::تحديث => طريقة::تحديث,
            طريقة_الطلب::حذف => طريقة::حذف,
            طريقة_الطلب::تعديل => طريقة::تعديل,
            طريقة_الطلب::خيارات => طريقة::خيارات,
            _ => طريقة::الكل,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المسار
// ═══════════════════════════════════════════════════════════════════════════════

/// مسار واحد
#[derive(Clone)]
pub struct مسار {
    /// النمط (مثل "مستخدم/{معرف}")
    pub النمط: String,
    /// الطريقة
    pub الطريقة: طريقة,
    /// المعالج
    pub المعالج: معالج,
    /// أجزاء النمط (للمطابقة السريعة)
    pub الأجزاء: Vec<جزء_المسار>,
}

/// جزء من المسار
#[derive(Debug, Clone)]
pub enum جزء_المسار {
    /// نص ثابت
    ثابت(String),
    /// معامل متغير
    معامل(String),
    /// معامل اختياري
    اختياري(String),
    /// wildcard (*)
    wildcard,
}

impl مسار {
    /// إنشاء مسار جديد
    pub fn جديد<S: Into<String>>(النمط: S, الطريقة: طريقة, المعالج: معالج) -> Self {
        let pattern = النمط.into();
        let parts = Self::تحليل_النمط(&pattern);
        
        مسار {
            النمط: pattern,
            الطريقة,
            المعالج,
            الأجزاء: parts,
        }
    }
    
    /// تحليل نمط المسار إلى أجزاء
    fn تحليل_النمط(النمط: &str) -> Vec<جزء_المسار> {
        النمط
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|part| {
                if part == "*" {
                    جزء_المسار::wildcard
                } else if part.starts_with('{') && part.ends_with('}') {
                    let name = &part[1..part.len()-1];
                    if name.ends_with('?') {
                        جزء_المسار::اختياري(name[..name.len()-1].to_string())
                    } else {
                        جزء_المسار::معامل(name.to_string())
                    }
                } else {
                    جزء_المسار::ثابت(part.to_string())
                }
            })
            .collect()
    }
    
    /// مطابقة المسار مع طلب
    pub fn طابق(&self, المسار: &str) -> Option<HashMap<String, String>> {
        let path_parts: Vec<&str> = المسار
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut params = HashMap::new();
        let mut path_idx = 0;
        
        for part in &self.الأجزاء {
            match part {
                جزء_المسار::ثابت(text) => {
                    if path_idx >= path_parts.len() || path_parts[path_idx] != text {
                        return None;
                    }
                    path_idx += 1;
                }
                جزء_المسار::معامل(name) => {
                    if path_idx >= path_parts.len() {
                        return None;
                    }
                    params.insert(name.clone(), path_parts[path_idx].to_string());
                    path_idx += 1;
                }
                جزء_المسار::اختياري(name) => {
                    if path_idx < path_parts.len() {
                        params.insert(name.clone(), path_parts[path_idx].to_string());
                        path_idx += 1;
                    }
                }
                جزء_المسار::wildcard => {
                    // يقبل أي شيء متبقي
                    for i in path_idx..path_parts.len() {
                        params.insert(format!("wildcard_{}", i), path_parts[i].to_string());
                    }
                    return Some(params);
                }
            }
        }
        
        // التحقق من استنفاد أجزاء المسار
        if path_idx == path_parts.len() {
            Some(params)
        } else {
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الموجّه
// ═══════════════════════════════════════════════════════════════════════════════

/// موجّه المسارات
pub struct الموجه {
    /// المسارات
    المسارات: RwLock<Vec<مسار>>,
    /// معالج 404
    معالج_404: RwLock<Option<معالج>>,
    /// مجموعة مسارات فرعية
    المجموعات: RwLock<HashMap<String, الموجه>>,
}

impl الموجه {
    /// إنشاء موجه جديد
    pub fn جديد() -> Self {
        الموجه {
            المسارات: RwLock::new(Vec::new()),
            معالج_404: RwLock::new(None),
            المجموعات: RwLock::new(HashMap::new()),
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // إضافة المسارات
    // ─────────────────────────────────────────────────────────────────────
    
    /// إضافة مسار GET
    pub fn جلب<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::جلب, المعالج);
        self
    }
    
    /// إضافة مسار POST
    pub fn إرسال<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::إرسال, المعالج);
        self
    }
    
    /// إضافة مسار PUT
    pub fn تحديث<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::تحديث, المعالج);
        self
    }
    
    /// إضافة مسار DELETE
    pub fn حذف<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::حذف, المعالج);
        self
    }
    
    /// إضافة مسار PATCH
    pub fn تعديل<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::تعديل, المعالج);
        self
    }
    
    /// إضافة مسار لجميع الطرق
    pub fn الكل<F>(&self, النمط: &str, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.أضف_مسار(النمط, طريقة::الكل, المعالج);
        self
    }
    
    /// إضافة مسار مخصص
    fn أضف_مسار<F>(&self, النمط: &str, الطريقة: طريقة, المعالج: F)
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        let route = مسار::جديد(النمط, الطريقة, Arc::new(المعالج));
        self.المسارات.write().push(route);
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // التوجيه المتقدم
    // ─────────────────────────────────────────────────────────────────────
    
    /// مجموعة مسارات (بادئة مشتركة)
    pub fn مجموعة<F>(&self, البادئة: &str, المعالج: F) -> &Self
    where
        F: FnOnce(&الموجه),
    {
        let sub_router = الموجه::جديد();
        المعالج(&sub_router);
        self.المجموعات.write().insert(البادئة.to_string(), sub_router);
        self
    }
    
    /// تعيين معالج 404
    pub fn غير_موجود<F>(&self, المعالج: F) -> &Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        *self.معالج_404.write() = Some(Arc::new(المعالج));
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // المعالجة
    // ─────────────────────────────────────────────────────────────────────
    
    /// معالجة طلب
    pub fn عالج(&self, طلب_م: طلب) -> رد {
        let path = طلب_م.المسار_النظيف().to_string();
        let method: طريقة = طلب_م.الطريقة.into();
        
        // البحث في المسارات
        let routes = self.المسارات.read();
        for route in routes.iter() {
            // التحقق من الطريقة
            if route.الطريقة != طريقة::الكل && route.الطريقة != method {
                continue;
            }
            
            // التحقق من النمط
            if let Some(params) = route.طابق(&path) {
                let mut request = طلب_م;
                request.المعاملات = params;
                return (route.المعالج)(request);
            }
        }
        drop(routes);
        
        // البحث في المجموعات الفرعية
        let groups = self.المجموعات.read();
        for (prefix, sub_router) in groups.iter() {
            if path.starts_with(prefix) {
                let mut sub_request = طلب_م;
                sub_request.المسار = path[prefix.len()..].to_string();
                return sub_router.عالج(sub_request);
            }
        }
        drop(groups);
        
        // معالج 404
        if let Some(handler) = self.معالج_404.read().as_ref() {
            return handler(طلب_م);
        }
        
        // رد 404 افتراضي
        رد::غير_موجود("الصفحة غير موجودة")
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // معلومات
    // ─────────────────────────────────────────────────────────────────────
    
    /// عدد المسارات
    pub fn عدد_المسارات(&self) -> usize {
        self.المسارات.read().len()
    }
    
    /// جميع المسارات
    pub fn جميع_المسارات(&self) -> Vec<String> {
        self.المسارات.read().iter().map(|r| r.النمط.clone()).collect()
    }
}

impl Default for الموجه {
    fn default() -> Self {
        الموجه::جديد()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod اختبارات {
    use super::*;
    
    #[test]
    fn اختبار_مطابقة_المسار() {
        let route = مسار::جديد("مستخدم/{معرف}", طريقة::جلب, Arc::new(|_| رد::نص("ok")));
        
        let result = route.طابق("مستخدم/123");
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("معرف").unwrap(), "123");
        
        let result = route.طابق("مستخدم");
        assert!(result.is_none());
    }
    
    #[test]
    fn اختبار_الموجه() {
        let router = الموجه::جديد();
        router.جلب("مرحبا", |_| رد::نص("مرحباً بالعالم!"));
        
        let request = طلب {
            الطريقة: طريقة_الطلب::جلب,
            المسار: "/مرحبا".to_string(),
            ..طلب::default()
        };
        
        let response = router.عالج(request);
        assert_eq!(response.كنص(), "مرحباً بالعالم!");
    }
}
