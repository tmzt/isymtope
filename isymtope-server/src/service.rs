use hyper::Request;

pub trait IsymtopeAppServiceFactory {
    type Request;
    type Response;
    type Error;
    type Instance;

    fn create(&self) -> Self::Instance;
}

pub trait IsymtopeAppService {
    type Request;
    type Response;
    type Error;
    type Future;

    fn call(&self, base_url: &str, app_name: &str, req: Request) -> Self::Future;
}
