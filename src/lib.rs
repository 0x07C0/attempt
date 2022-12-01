//! Solution to the test problem provided by Out of The Box Systems
//!
//! Provides the `attempt` function which returns a vector of
//! values filtered by parameters

use std::cmp::Ordering;

/// Describes value used by `allowed` and `preferred` vectors.
#[ derive( PartialEq ) ]
pub enum Value 
{
    /// For `allowed` cancels filtering of `avaliable` vector.
    ///
    /// For `preferred` cancels disables reducing by number of preferences.
    Any,
    /// Stores the regular value, unused if vector contains `Value::Any` alongside.
    Number( i32 ),
}

/// Reduces amount of available values by `allowed` filter vector.
///
/// Returns vector with values both `original` and `allowed` have.
fn reduce_by_allowed( original : &Vec< i32 >, allowed : &Vec< Value > ) -> Vec< i32 > 
{
    let mut o_pointer = 0;
    let mut a_pointer = 0;
    let mut vec : Vec< i32 > = vec![];
    while o_pointer < original.len() && a_pointer < allowed.len() 
    {
        if let Value::Number( num ) = &allowed[ a_pointer ]
        {
            match original[ o_pointer ].cmp( num )
            {
                Ordering::Less => o_pointer += 1,
                Ordering::Greater => a_pointer += 1,
                Ordering::Equal =>
                {
                    vec.push( original[ o_pointer ] );
                    o_pointer += 1;
                    a_pointer += 1;
                }
            }
        }
    }
    vec
}

/// Reduces original array by `preferred` vector and its size.
///
/// Returns vector with values that closest to `preferred` values.
fn reduce_by_preferred( original : &Vec< i32 >, preferred : &Vec< Value > ) -> Vec< i32 > 
{
    let mut o_pointer = 0;
    let mut p_pointer = 0;
    let mut vec : Vec< i32 > = vec![];
    while o_pointer < original.len() && p_pointer < preferred.len() {
        if let Value::Number( num ) = &preferred[ p_pointer ] {
            match original[ o_pointer ].cmp( num ) {
                Ordering::Less => 
                {
                    o_pointer += 1;
                    if o_pointer >= original.len() 
                    {
                        vec.push( original[ o_pointer - 1 ] )
                    }
                }
                Ordering::Greater => 
                {
                    vec.push( original[ o_pointer ] );
                    p_pointer += 1;
                }
                Ordering::Equal => 
                {
                    vec.push( original[ o_pointer ] );
                    o_pointer += 1;
                    p_pointer += 1;
                }
            }
        }
    }

    vec
}

/// Reduces `avaliable` vector by both `allowed` and `preferred` vector filters.
///
/// Returns vector of values that are both inside `avaliable` and `allowed` vectors, and
/// has a size equal or less of `preferred` vector.
///
/// Returns an empty vector if none of the `allowed` values are inside of `available` vector.
///
/// # Examples
///
/// ```
/// use attempt::*;
///
/// assert_eq!(
///     attempt(
///         &vec![ 240, 360, 720 ],                            // available values
///         &vec![ Value::Number( 360 ), Value::Number( 720 ) ], // allowed filter vector
///         &vec![ Value::Number( 1080 ) ]                     // preferences
///     ),
///     vec![ 720 ]           // should return the value closest to 1080 and that is allowed
/// );
///
/// assert_eq!(
///     attempt(
///         &vec![ 240, 360, 720 ],
///         &vec![ Value::Number( 360 ), Value::Any ],
///         &vec![ Value::Number( 360 ), Value::Number( 720 ) ]
///     ),
///     vec![ 360, 720 ]
/// );
/// assert_eq!(
///     attempt(
///         &vec![ 240, 360, 720 ],
///         &vec![ Value::Number( 240 ), Value::Number( 360 ), Value::Number( 720 ) ],
///         &vec![ Value::Any, Value::Number( 720 ) ]
///     ),
///     vec![ 240, 360, 720 ]
/// );
/// ```
pub fn attempt(available : &Vec< i32 >, allowed : &Vec< Value >, preferred : &Vec< Value >) -> Vec< i32 >
{
    if allowed.contains( &Value::Any ) && preferred.contains( &Value::Any ) 
    {
        return available.to_vec();
    }
    if allowed.contains( &Value::Any ) 
    {
        return reduce_by_preferred( available, preferred );
    }
    if preferred.contains( &Value::Any ) 
    {
        return reduce_by_allowed( available, allowed );
    }
    let available = &reduce_by_allowed( available, allowed );
    reduce_by_preferred( available, preferred )
}

#[ cfg( test ) ]
mod tests
{
    use super::*;

    #[ test ]
    fn reducing_by_allowed() 
    {
        assert_eq!(
            reduce_by_allowed(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ]
            ),
            vec![ 360, 720 ]
        );

        assert_eq!(
            reduce_by_allowed(
                &vec![ 240, 720 ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ]
            ),
            vec![ 720 ]
        );

        assert_eq!(
            reduce_by_allowed( &vec![ 240 ], &vec![ Value::Number( 360 ), Value::Number( 720 ) ] ),
            vec![]
        );
    }

    #[test]
    fn reducing_by_preferred() 
    {
        assert_eq!(
            reduce_by_preferred( &vec![ 240, 360, 720 ], &vec![ Value::Number( 360 ) ] ),
            vec![ 360 ]
        );

        assert_eq!(
            reduce_by_preferred( &vec![ 360, 720 ], &vec![ Value::Number( 1080 ) ] ),
            vec![ 720 ]
        );

        assert_eq!(
            reduce_by_preferred(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![ 240, 360 ]
        );
        assert_eq!(
            reduce_by_preferred(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 1080 ) ]
            ),
            vec![ 240, 720 ]
        );
        assert_eq!(
            reduce_by_preferred(
                &vec![ 240, 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![ 240, 720 ]
        );
    }

    #[test]
    fn test_attempt() 
    {
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ],
                &vec![ Value::Number( 1080 ) ]
            ),
            vec![ 720 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 720 ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ],
                &vec![ Value::Number( 1080 ) ]
            ),
            vec![ 720 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240 ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ],
                &vec![ Value::Number( 1080 ) ]
            ),
            vec![]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![
                    Value::Number( 240 ),
                    Value::Number( 360 ),
                    Value::Number( 720 ),
                    Value::Number( 1080 )
                ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![ 240, 360 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 720 ],
                &vec![
                    Value::Number( 240 ),
                    Value::Number( 360 ),
                    Value::Number( 720 ),
                    Value::Number( 1080 )
                ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![ 240, 720 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ), Value::Number( 1080 ) ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![ 240 ]
        );
        assert_eq!(
            attempt(
                &vec![ 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ), Value::Number( 1080 ) ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ]
            ),
            vec![]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 360 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ) ],
                &vec![ Value::Number( 720 ), Value::Number( 1080 ) ]
            ),
            vec![ 360 ]
        );
    }

    #[test]
    fn test_attempt_with_any() 
    {
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 360 ), Value::Any ],
                &vec![ Value::Number( 360 ), Value::Number( 720 ) ]
            ),
            vec![ 360, 720 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 240 ), Value::Number( 360 ), Value::Number( 720 ) ],
                &vec![ Value::Any, Value::Number( 720 ) ]
            ),
            vec![ 240, 360, 720 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 360 ), Value::Number( 1080 ) ],
                &vec![ Value::Any, Value::Number( 720 ) ]
            ),
            vec![ 360 ]
        );
        assert_eq!(
            attempt(
                &vec![ 240, 360, 720 ],
                &vec![ Value::Number( 1080 ) ],
                &vec![ Value::Any, Value::Number( 720 ) ]
            ),
            vec![]
        );
        assert_eq!(
            attempt( &vec![ 240, 360, 720 ], &vec![ Value::Any ], &vec![ Value::Any ] ),
            vec![ 240, 360, 720 ]
        );
    }
}
