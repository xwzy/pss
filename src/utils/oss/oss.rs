use oss_rust_sdk::prelude::*;

mod oss {
    pub fn list_buckets() {
        let oss_instance = OSS::new(
            "your_AccessKeyId",
            "your_AccessKeySecret",
            "your_Endpoint",
            "your_Bucket",
        );
        let list_your_Buckets = oss_instance.list_your_Bucket(None).unwrap();
    
        let id = list_your_Buckets.id();
        let your_Buckets = list_your_Buckets.your_Buckets();
        let your_Bucket_names: Vec<&str> = your_Buckets.iter().map(|obj| obj.name()).collect();
    }
}

