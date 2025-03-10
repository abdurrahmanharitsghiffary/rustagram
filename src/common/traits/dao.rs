pub struct PaginationOptions {
    pub per_page: u32,
    pub page: u32,
}

pub enum OrderBy {
    ASC,
    DESC,
}

pub struct OrderOptions {
    pub order_column: String,
    pub order_by: OrderBy,
}

pub enum WhereOptions {
    EQ,
    NotEQ,
    GT,
    GTE,
    LT,
    LTE,
    In,
    NotIn,
    Between,
    NotBetween,
}

pub enum LikeValueOptions {
    StartWiths,
    EndWiths,
    Contains,
}

pub enum LikeOptions {
    Like(LikeValueOptions),
    NotLike(LikeValueOptions),
    ILike(LikeValueOptions),
    INotLike(LikeValueOptions),
    Fulltext,
}

pub struct DaoFilter {
    pub pagination: Option<PaginationOptions>,
    pub order_options: Option<OrderOptions>,
    pub where_condition: Vec<(String, WhereOptions, String)>,
    pub like_condition: Vec<(String, LikeOptions, String)>,
}

pub trait Dao<T, ID> {
    async fn find_all(filters: Option<DaoFilter>) -> (Vec<T>, u64);
    async fn create(data: T) -> ();
    async fn update(id: ID, updated: T) -> T;
    async fn find_one(id: ID) -> Option<T>;
    async fn delete(id: ID) -> T;
    async fn count(filters: Option<DaoFilter>) -> u64;
}
