use mysql_async::prelude::FromRow;
use mysql_async::prelude::{ConvIr, FromValue};
use mysql_async::{from_row, FromRowError, FromValueError, Row, Value};
use num::{FromPrimitive, ToPrimitive};
use crate::{take_opt_or_place, take_or_place};

#[derive(Deserialize, Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum Sex {
    Male = 0,
    Female,
}

pub struct AdminPo {
    pub sqn: u32,
    pub account: String,
    pub pass: String,
    pub status: AdminStatus,
    pub name: String,
    pub sex: Sex,
    pub mobile: String,
    pub email: String,
    pub icon: String,
    pub remark: String,
    pub ctime: String,
}

//参数个数超过12个，需要自行处理，如下。
//<=12 可以使用from_row。 @see CustomerServicePo
impl FromRow for AdminPo {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 11 {
            return Err(FromRowError(row));
        }
        let mut r = row;
        let (
            sqn,
            account,
            pass,
            status,
            name,
            sex,
            mobile,
            email,
            icon,
            remark,
            ctime,
        ) = (
            take_or_place!(r, 0, u32),
            take_or_place!(r, 1, String),
            take_or_place!(r, 2, String),
            take_or_place!(r, 3, i8),
            take_or_place!(r, 4, String),
            take_or_place!(r, 5, i8),
            take_or_place!(r, 6, String),
            take_or_place!(r, 7, String),
            take_or_place!(r, 8, String),
            take_or_place!(r, 9, String),
            take_or_place!(r, 10, String),
        );
        Ok(Self {
            sqn,
            account,
            pass,
            status: AdminStatus::from_i8(status)
                .unwrap_or(AdminStatus::Normal),
            name,
            sex: Sex::from_i8(sex).unwrap_or(Sex::Male),
            mobile,
            email,
            icon,
            remark,
            ctime,
        })
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum AdminStatus {
    Normal = 0,
    Denied,
    Cancelled,
}

impl From<AdminStatus> for String {
    fn from(s: AdminStatus) -> Self {
        String::from(match s {
            AdminStatus::Normal => "正常",
            AdminStatus::Denied => "禁用",
            AdminStatus::Cancelled => "注销",
        })
    }
}

pub struct CustomerServicePo {
    pub sqn: u32,
    pub wechat: String,
    pub mobile: String,
    pub address: String,
}

impl FromRow for CustomerServicePo {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let (sqn, wechat, mobile, address) =
            from_row::<(u32, Option<String>, Option<String>, Option<String>)>(row);
        Ok(Self {
            sqn,
            wechat: wechat.unwrap_or(String::new()),
            mobile: mobile.unwrap_or(String::new()),
            address: address.unwrap_or(String::new()),
        })
    }
}