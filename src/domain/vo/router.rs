#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaVO {
    //设置该路由在侧边栏和面包屑中展示的名字
    pub title: Option<String>,
    //设置该路由的图标，对应路径src/assets/icons/svg
    pub icon: Option<String>,
    //设置为true，则不会被 <keep-alive>缓存
    pub no_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    //内链地址（http(s)://开头）
    pub link: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterVO {
    //路由名字
    pub name:  Option<String>,
    //路由地址
    pub path:  Option<String>,
    //是否隐藏路由，当设置 true 的时候该路由不会再侧边栏出现
    pub hidden: Option<bool>,
    //重定向地址，当设置 noRedirect 的时候该路由在面包屑导航中不可被点击
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    //组件地址
    pub component: Option<String>,
    //路由参数：如 {"id": 1, "name": "ry"}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    //当你一个路由下面的 children 声明的路由大于1个时，自动会变成嵌套的模式--如组件页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show: Option<bool>,
    //其他元素
    pub meta: Option<MetaVO>,
    //子路由
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<RouterVO>,
}
