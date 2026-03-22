// ═══════════════════════════════════════════════════════════════════════════════
// القوالب - Templates
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// القالب
// ═══════════════════════════════════════════════════════════════════════════════

/// قالب HTML
pub struct قالب {
    /// المحتوى
    المحتوى: String,
    /// المتغيرات
    المتغيرات: HashMap<String, String>,
}

impl قالب {
    /// إنشاء قالب جديد
    pub fn جديد(المحتوى: &str) -> Self {
        قالب {
            المحتوى: المحتوى.to_string(),
            المتغيرات: HashMap::new(),
        }
    }
    
    /// إنشاء قالب من ملف
    pub fn من_ملف(المسار: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(المسار)
            .map_err(|e| format!("خطأ في قراءة الملف: {}", e))?;
        
        Ok(قالب::جديد(&content))
    }
    
    /// إضافة متغير
    pub fn متغير(mut self, الاسم: &str, القيمة: &str) -> Self {
        self.المتغيرات.insert(الاسم.to_string(), القيمة.to_string());
        self
    }
    
    /// إضافة متغيرات متعددة
    pub fn متغيرات(mut self, المتغيرات: HashMap<&str, &str>) -> Self {
        for (key, value) in المتغيرات {
            self.المتغيرات.insert(key.to_string(), value.to_string());
        }
        self
    }
    
    /// معالجة القالب
    pub fn معالجة(&self) -> String {
        let mut result = self.المحتوى.clone();
        
        // استبدال المتغيرات {{اسم}}
        for (key, value) in &self.المتغيرات {
            let pattern = format!("{{{{{}}}}}", key);
            result = result.replace(&pattern, value);
        }
        
        result
    }
    
    /// تحويل إلى رد HTML
    pub fn إلى_رد(&self) -> crate::response::رد {
        crate::response::رد::html(self.معالجة())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// قوالب جاهزة
// ═══════════════════════════════════════════════════════════════════════════════

/// قوالب HTML جاهزة
pub struct قوالب;

impl قوالب {
    /// صفحة بسيطة
    pub fn صفحة_بسيطة(العنوان: &str, المحتوى: &str) -> crate::response::رد {
        let html = format!(r#"
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', Tahoma, Arial, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #eee;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .container {{
            text-align: center;
            padding: 2rem;
        }}
        h1 {{
            font-size: 2.5rem;
            margin-bottom: 1rem;
            color: #00d9ff;
        }}
        p {{
            font-size: 1.2rem;
            opacity: 0.8;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>{}</h1>
        <p>{}</p>
    </div>
</body>
</html>
"#, العنوان, العنوان, المحتوى);
        
        crate::response::رد::html(html)
    }
    
    /// صفحة خطأ
    pub fn صفحة_خطأ(الكود: u16, الرسالة: &str) -> crate::response::رد {
        let html = format!(r#"
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>خطأ {}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', Tahoma, Arial, sans-serif;
            background: linear-gradient(135deg, #2d132c 0%, #801336 100%);
            color: #fff;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .error-container {{
            text-align: center;
            padding: 2rem;
        }}
        .error-code {{
            font-size: 8rem;
            font-weight: bold;
            color: #ff6b6b;
            text-shadow: 4px 4px 0 rgba(0,0,0,0.3);
        }}
        .error-message {{
            font-size: 1.5rem;
            margin-top: 1rem;
            opacity: 0.9;
        }}
        a {{
            display: inline-block;
            margin-top: 2rem;
            padding: 1rem 2rem;
            background: #ff6b6b;
            color: white;
            text-decoration: none;
            border-radius: 5px;
            transition: transform 0.2s;
        }}
        a:hover {{ transform: scale(1.05); }}
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-code">{}</div>
        <div class="error-message">{}</div>
        <a href="/">العودة للرئيسية</a>
    </div>
</body>
</html>
"#, الكود, الكود, الرسالة);
        
        crate::response::رد::html(html)
    }
    
    /// صفحة API
    pub fn صفحة_api(العنوان: &str, الوصف: &str) -> crate::response::رد {
        let html = format!(r#"
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', Tahoma, Arial, sans-serif;
            background: #0f0f23;
            color: #ccc;
            min-height: 100vh;
            padding: 2rem;
        }}
        .header {{
            text-align: center;
            margin-bottom: 2rem;
        }}
        h1 {{
            color: #00d9ff;
            margin-bottom: 0.5rem;
        }}
        p {{ opacity: 0.7; }}
        .endpoint {{
            background: #1a1a2e;
            padding: 1rem;
            margin: 1rem 0;
            border-radius: 8px;
            border-left: 4px solid #00d9ff;
        }}
        .method {{
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 4px;
            font-weight: bold;
            margin-left: 0.5rem;
        }}
        .get {{ background: #2ecc71; }}
        .post {{ background: #3498db; }}
        .put {{ background: #f39c12; }}
        .delete {{ background: #e74c3c; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🌙 {}</h1>
        <p>{}</p>
    </div>
</body>
</html>
"#, العنوان, العنوان, الوصف);
        
        crate::response::رد::html(html)
    }
}
