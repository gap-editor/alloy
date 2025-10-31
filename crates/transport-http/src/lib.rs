--- a/http.rs
+++ b/http.rs
@@
 impl<T> FromStr for HttpConnect<T> {
     type Err = url::ParseError;
 
     fn from_str(s: &str) -> Result<Self, Self::Err> {
         Ok(Self::new(s.parse()?))
     }
 }
 
-// Дублирующая конверсия: строка -> HttpConnect<T>
-impl<T> TryFrom<&str> for HttpConnect<T> {
-    type Error = url::ParseError;
-    fn try_from(s: &str) -> Result<Self, Self::Error> {
-        s.parse()
-    }
-}
-
-// Дублирующая конверсия: HttpConnect<T> -> Url
-impl<T> From<HttpConnect<T>> for Url {
-    fn from(conn: HttpConnect<T>) -> Self {
-        conn.url
-    }
-}
