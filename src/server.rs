// ═══════════════════════════════════════════════════════════════════════════════
// الخادم - Server
// ═══════════════════════════════════════════════════════════════════════════════

use crate::error::خطأ_ويب;
use crate::middleware::سلسلة_الوسائط;
use crate::request::{طلب, طريقة_الطلب};
use crate::response::{رد, حالة};
use crate::router::{الموجه, طريقة};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use parking_lot::RwLock;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// ═══════════════════════════════════════════════════════════════════════════════
// إعدادات الخادم
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات الخادم
#[derive(Debug, Clone)]
pub struct إعدادات_الخادم {
    /// المنفذ
    pub المنفذ: u16,
    /// المضيف
    pub المضيف: String,
    /// عدد العمال
    pub عدد_العمال: usize,
    /// الحد الأقصى للاتصالات
    pub حد_الاتصالات: usize,
    /// Timeout بالثواني
    pub المهلة: u64,
    /// تفعيل CORS
    pub تفعيل_cors: bool,
    /// أصول CORS المسموحة
    pub أصول_cors: Vec<String>,
    /// تفعيل الضغط
    pub تفعيل_الضغط: bool,
    /// الحد الأقصى لحجم الطلب (بالبايت)
    pub حد_حجم_الطلب: usize,
}

impl Default for إعدادات_الخادم {
    fn default() -> Self {
        إعدادات_الخادم {
            المنفذ: 8080,
            المضيف: "0.0.0.0".to_string(),
            عدد_العمال: num_cpus::get(),
            حد_الاتصالات: 1024,
            المهلة: 30,
            تفعيل_cors: false,
            أصول_cors: Vec::new(),
            تفعيل_الضغط: true,
            حد_حجم_الطلب: 10 * 1024 * 1024, // 10 MB
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الخادم
// ═══════════════════════════════════════════════════════════════════════════════

/// خادم ويب عربي
pub struct خادم {
    /// الموجّه
    pub الموجه: Arc<الموجه>,
    /// الإعدادات
    pub الإعدادات: إعدادات_الخادم,
    /// الوسائط
    pub الوسائط: سلسلة_الوسائط,
    /// حالات الخادم
    running: RwLock<bool>,
}

impl خادم {
    /// إنشاء خادم جديد
    pub fn جديد() -> Self {
        خادم {
            الموجه: Arc::new(الموجه::جديد()),
            الإعدادات: إعدادات_الخادم::default(),
            الوسائط: سلسلة_الوسائط::جديد(),
            running: RwLock::new(false),
        }
    }
    
    /// إنشاء خادم مع إعدادات مخصصة
    pub fn مع_إعدادات(الإعدادات: إعدادات_الخادم) -> Self {
        خادم {
            الموجه: Arc::new(الموجه::جديد()),
            الإعدادات,
            الوسائط: سلسلة_الوسائط::جديد(),
            running: RwLock::new(false),
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // إضافة المسارات
    // ─────────────────────────────────────────────────────────────────────
    
    /// إضافة مسار GET
    pub fn مسار<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.جلب(النمط, المعالج);
        self
    }
    
    /// إضافة مسار GET
    pub fn جلب<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.جلب(النمط, المعالج);
        self
    }
    
    /// إضافة مسار POST
    pub fn إرسال<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.إرسال(النمط, المعالج);
        self
    }
    
    /// إضافة مسار PUT
    pub fn تحديث<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.تحديث(النمط, المعالج);
        self
    }
    
    /// إضافة مسار DELETE
    pub fn حذف<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.حذف(النمط, المعالج);
        self
    }
    
    /// إضافة مسار PATCH
    pub fn تعديل<F>(self, النمط: &str, المعالج: F) -> Self
    where
        F: Fn(طلب) -> رد + Send + Sync + 'static,
    {
        self.الموجه.تعديل(النمط, المعالج);
        self
    }
    
    /// إضافة وسیطة
    pub fn وسیطة<F>(mut self, المعالج: F) -> Self
    where
        F: Fn(طلب) -> Option<رد> + Send + Sync + 'static,
    {
        self.الوسائط.أضف(المعالج);
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // الإعدادات
    // ─────────────────────────────────────────────────────────────────────
    
    /// تغيير المنفذ
    pub fn منفذ(mut self, المنفذ: u16) -> Self {
        self.الإعدادات.المنفذ = المنفذ;
        self
    }
    
    /// تغيير المضيف
    pub fn مضيف(mut self, المضيف: &str) -> Self {
        self.الإعدادات.المضيف = المضيف.to_string();
        self
    }
    
    /// تفعيل CORS
    pub fn مع_cors(mut self) -> Self {
        self.الإعدادات.تفعيل_cors = true;
        self
    }
    
    /// تفعيل CORS مع أصول محددة
    pub fn مع_cors_محدد(mut self, الأصول: Vec<&str>) -> Self {
        self.الإعدادات.تفعيل_cors = true;
        self.الإعدادات.أصول_cors = الأصول.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// تغيير المهلة
    pub fn مهلة(mut self, الثواني: u64) -> Self {
        self.الإعدادات.المهلة = الثواني;
        self
    }
    
    /// تغيير حد حجم الطلب
    pub fn حد_الحجم(mut self, البايت: usize) -> Self {
        self.الإعدادات.حد_حجم_الطلب = البايت;
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // تشغيل الخادم
    // ─────────────────────────────────────────────────────────────────────
    
    /// تشغيل الخادم
    pub async fn شغل(self, المنفذ: u16) {
        let server = self.منفذ(المنفذ);
        server.ابدأ().await;
    }
    
    /// تشغيل الخادم مع الإعدادات الحالية
    pub async fn ابدأ(self) {
        let addr: SocketAddr = format!("{}:{}", self.الإعدادات.المضيف, self.الإعدادات.المنفذ)
            .parse()
            .expect("عنوان غير صالح");
        
        let listener = TcpListener::bind(addr)
            .await
            .expect("فشل ربط المنفذ");
        
        // طباعة رسالة البدء
        self.اطبع_رسالة_البدء();
        
        *self.running.write() = true;
        
        let router = Arc::clone(&self.الموجه);
        let middleware = Arc::new(self.الوسائط);
        let settings = Arc::new(self.الإعدادات);
        
        println!("🌐 الخادم يعمل على http://{}", addr);
        println!("📖 للإيقاف اضغط Ctrl+C");
        
        loop {
            let (stream, remote_addr) = listener.accept().await.expect("فشل قبول الاتصال");
            
            let io = TokioIo::new(stream);
            let router = Arc::clone(&router);
            let middleware = Arc::clone(&middleware);
            let settings = Arc::clone(&settings);
            
            tokio::spawn(async move {
                let service = service_fn(move |req: Request<Incoming>| {
                    let router = Arc::clone(&router);
                    let middleware = Arc::clone(&middleware);
                    let settings = Arc::clone(&settings);
                    
                    async move {
                        // تحويل الطلب
                        let request = تحويل_الطلب(req, remote_addr).await;
                        
                        // تطبيق الوسائط
                        if let Some(response) = middleware.نفذ(request.clone()) {
                            return Ok::<_, hyper::Error>(تحويل_الرد(response));
                        }
                        
                        // معالجة الطلب
                        let response = router.عالج(request);
                        
                        // إضافة CORS إذا كان مفعلاً
                        let response = if settings.تفعيل_cors {
                            response.مع_cors()
                        } else {
                            response
                        };
                        
                        Ok(تحويل_الرد(response))
                    }
                });
                
                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("خطأ في الاتصال: {}", err);
                }
            });
        }
    }
    
    /// طباعة رسالة البدء
    fn اطبع_رسالة_البدء(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║           🌙 خادم المرجع - Al-Marjaa Web Server 🌙           ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║  الإصدار: 3.4.0                                              ║");
        println!("║  المنفذ: {}                                                  ║", self.الإعدادات.المنفذ);
        println!("║  المسارات: {}                                                ║", self.الموجه.عدد_المسارات());
        println!("║  CORS: {}                                                    ║", 
            if self.الإعدادات.تفعيل_cors { "مفعل ✓" } else { "معطل ✗" });
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
    }
}

impl Default for خادم {
    fn default() -> Self {
        خادم::جديد()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال التحويل
// ═══════════════════════════════════════════════════════════════════════════════

/// تحويل طلب Hyper إلى طلبنا
async fn تحويل_الطلب(req: Request<Incoming>, remote_addr: SocketAddr) -> طلب {
    let method = match req.method() {
        &Method::GET => طريقة_الطلب::جلب,
        &Method::POST => طريقة_الطلب::إرسال,
        &Method::PUT => طريقة_الطلب::تحديث,
        &Method::DELETE => طريقة_الطلب::حذف,
        &Method::PATCH => طريقة_الطلب::تعديل,
        &Method::HEAD => طريقة_الطلب::رأس,
        &Method::OPTIONS => طريقة_الطلب::خيارات,
        &Method::CONNECT => طريقة_الطلب::اتصال,
        &Method::TRACE => طريقة_الطلب::تتبع,
        _ => طريقة_الطلب::جلب,
    };
    
    let uri = req.uri().to_string();
    let path = req.uri().path().to_string();
    
    // استخراج معاملات الاستعلام
    let mut query = HashMap::new();
    if let Some(query_str) = req.uri().query() {
        for pair in query_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                query.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    // استخراج الرؤوس
    let mut headers = HashMap::new();
    for (name, value) in req.headers() {
        if let Ok(v) = value.to_str() {
            headers.insert(name.to_string(), v.to_string());
        }
    }
    
    // استخراج الكوكيز
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get("Cookie") {
        for pair in cookie_header.split(';') {
            if let Some((key, value)) = pair.trim().split_once('=') {
                cookies.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    // استخراج المحتوى
    let body = req.collect().await.expect("فشل قراءة المحتوى");
    let content = body.to_bytes().to_vec();
    
    طلب {
        الطريقة: method,
        المسار: path,
        المعاملات: HashMap::new(),
        الاستعلام: query,
        الرؤوس: headers,
        المحتوى: content,
        عنوان_العميل: Some(remote_addr),
        الكوكيز: cookies,
    }
}

/// تحويل ردنا إلى رد Hyper
fn تحويل_الرد(رد: رد) -> Response<Full<Bytes>> {
    let mut builder = Response::builder()
        .status(رد.حالة.الكود);
    
    // إضافة نوع المحتوى
    builder = builder.header("Content-Type", رد.نوع_المحتوى);
    
    // إضافة الرؤوس
    for (name, value) in &رد.رؤوس {
        builder = builder.header(name, value);
    }
    
    // إضافة الرؤوس الأساسية
    builder = builder
        .header("X-Powered-By", "Al-Marjaa-Web/3.4.0")
        .header("Server", "Al-Marjaa");
    
    builder.body(Full::new(Bytes::from(رد.المحتوى)))
        .expect("فشل إنشاء الرد")
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod اختبارات {
    use super::*;
    
    #[test]
    fn اختبار_إنشاء_خادم() {
        let server = خادم::جديد();
        assert_eq!(server.الإعدادات.المنفذ, 8080);
    }
    
    #[test]
    fn اختبار_تغيير_المنفذ() {
        let server = خادم::جديد().منفذ(3000);
        assert_eq!(server.الإعدادات.المنفذ, 3000);
    }
}
