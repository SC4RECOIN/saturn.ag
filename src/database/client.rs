use postgrest::Postgrest;

pub fn get_db_client() -> Postgrest {
    // key for anon user (RLS)
    Postgrest::new("https://nooxxwistdfdfbudtumk.supabase.co/rest/v1")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im5vb3h4d2lzdGRmZGZidWR0dW1rIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Mzg1MDkyMTMsImV4cCI6MjA1NDA4NTIxM30.9XFXAZyWiohYwxPpwpg0GTLLOahKmcEGDzffYx7ziYY")
}
