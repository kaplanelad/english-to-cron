use english_to_cron::str_cron_syntax;
use rstest::rstest;

#[rstest]
// Seconds
#[case("Run second", Ok("* * * * * ? *"))]
#[case("every 5 second", Ok("0/5 * * * * ? *"))]
#[case("every 5 second on september", Ok("0/5 * * * SEP ? *"))]
#[case("every 5 second on 9 month", Ok("0/5 * * * 9 ? *"))]
#[case("Every 2 seconds, only on thursday", Ok("0/2 * * ? * THU *"))]
#[case("Run every 2 second on the 12th day", Ok("0/2 0 0 12 * ? *"))]
#[case("Run every 2 second on Monday thursday", Ok("0/2 * * ? * MON,THU *"))]
#[case(
    "Run every 10 seconds Monday through thursday between 6:00 am and 8:00 pm",
    Ok("0/10 * 6-20 ? * MON-THU *")
)]
// Minutes
#[case("Run every minute", Ok("0 * * * * ? *"))]
#[case("Run every 15 minutes", Ok("0 0/15 * * * ? *"))]
#[case("every minutes on thursday", Ok("0 * * ? * THU *"))]
#[case("every 2 minutes on Thursday", Ok("0 0/2 * ? * THU *"))]
#[case(
    "Run every 10 minutes Monday through Friday every month",
    Ok("0 0/10 * ? * MON-FRI *")
)]
#[case(
    "Run every 1 minutes Monday through Thursday between 6:00 am and 9:00 pm",
    Ok("0 0/1 6-21 ? * MON-THU *")
)]
#[case(
    "Run every 5 minutes Monday through Thursday between 6:00 am and 9:00 am",
    Ok("0 0/5 6-9 ? * MON-THU *")
)]
#[case("Every 5 minutes, only on Friday", Ok("0 0/5 * ? * FRI *"))]
// Hours
#[case("Run every 3 hours", Ok("0 0 0/3 * * ? *"))]
#[case(
    "Run every 6 hours, starting at 1:00 pm on day Monday",
    Ok("0 0 0/6 ? * MON *")
)]
#[case("Run every 1 hour only on weekends", Ok("0 0 0/1 ? * SAT,SUN *"))]
#[case("Run every hour only on weekends", Ok("0 0 * ? * SAT,SUN *"))]
#[case(
    "2pm on Tuesday, Wednesday and Thursday",
    Ok("0 0 14 ? * TUE,WED,THU *")
)]
// Days
#[case("Run every day", Ok("0 0 0 */1 * ? *"))]
#[case("Run every 4 days", Ok("0 0 0 */4 * ? *"))]
#[case("every day at 4:00 pm", Ok("0 0 16 */1 * ? *"))]
#[case("every 2 day at 4:00 pm", Ok("0 0 16 */2 * ? *"))]
#[case("every 5 day at 4:30 pm", Ok("0 30 16 */5 * ? *"))]
#[case("every 5 day at 4:30 pm only in September", Ok("0 30 16 */5 SEP ? *"))]
#[case(
    "every 5 day at 4:30 pm Monday through Thursday",
    Ok("0 30 16 ? * MON-THU *")
)]
#[case("Run every day from January to March", Ok("0 0 0 */1 JAN-MAR ? *"))]
#[case("Run every 3 days at noon", Ok("0 0 12 */3 * ? *"))]
#[case("Run every 2nd day of the month", Ok("0 0 0 2 * ? *"))]
// Month
#[case("Run every sec from January to March", Ok("* * * * JAN-MAR ? *"))]
#[case("Run every minute from January to March", Ok("0 * * * JAN-MAR ? *"))]
#[case("Run every hours from January to March", Ok("0 0 * * JAN-MAR ? *"))]
// Year
#[case(
    "every 2 day from January to August in 2020 and 2024",
    Ok("0 0 0 */2 JAN-AUG ? 2020,2024")
)]
// Specific Times (AM/PM)
#[case("Run at 10:00 am", Ok("0 0 10 * * ? *"))]
#[case("Run at 12:15 pm", Ok("0 15 12 * * ? *"))]
#[case(
    "Run at 6:00 pm every Monday through Friday",
    Ok("0 0 18 ? * MON-FRI *")
)]
#[case("Run at noon every Sunday", Ok("0 0 12 ? * SUN *"))]
#[case(
    "Run at midnight on the 1st and 15th of the month",
    Ok("0 0 0 1,15 * ? *")
)]
#[case("midnight on Tuesdays", Ok("0 0 0 ? * TUE *"))]
#[case("Run at 5:15am every Tuesday", Ok("0 15 5 ? * TUE *"))]
#[case("7pm every Thursday", Ok("0 0 19 ? * THU *"))]
#[case("2pm and 6pm", Ok("0 0 14,18 * * ? *"))]
#[case("5am, 10am and 3pm", Ok("0 0 5,10,15 * * ? *"))]
#[case("Run every hour only on Monday", Ok("0 0 * ? * MON *"))]
#[case("Run every 30 seconds only on weekends", Ok("0/30 * * ? * SAT,SUN *"))]
#[case("4pm, 5pm and 7pm", Ok("0 0 16,17,19 * * ? *"))]
#[case("4pm, 5pm, and 7pm", Ok("0 0 16,17,19 * * ? *"))]
#[case("4pm, 5pm, 7pm", Ok("0 0 16,17,19 * * ? *"))]
#[case("4pm and 5pm and 7pm", Ok("0 0 16,17,19 * * ? *"))]
#[test]
fn can_parse_string(
    #[case] cron_str: &str,
    #[case] expected_result: english_to_cron::Result<&str>,
) {
    let result = str_cron_syntax(cron_str);

    assert_eq!(
        result,
        expected_result
            .clone()
            .map(std::string::ToString::to_string),
        "Failed for input: '{cron_str}'. Expected: {expected_result:?}, Got: {result:?}"
    );
}
