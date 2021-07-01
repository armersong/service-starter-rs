pub mod admin;

pub const INVALID_SQN: u32 = 0;

#[macro_export]
macro_rules! take_or_place {
    ($row:expr, $index:expr, $t:ident) => {
        match $row.take($index) {
            Some(value) => match $t::get_intermediate(value) {
                Ok(ir) => ir.commit(),
                Err(FromValueError(value)) => {
                    $row.place($index, value);
                    return Err(FromRowError($row));
                }
            },
            None => return Err(FromRowError($row)),
        }
    };
}

#[macro_export]
macro_rules! take_opt_or_place {
    ($row:expr, $index:expr, $t:ident) => {
        match $row.take($index) {
            Some(value) => match value {
                Value::NULL => None,
                value => match $t::get_intermediate(value) {
                    Ok(ir) => Some(ir.commit()),
                    Err(FromValueError(value)) => {
                        $row.place($index, value);
                        return Err(FromRowError($row));
                    }
                },
            },
            None => return Err(FromRowError($row)),
        }
    };
}
