use super::*;

pub fn examples_time() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
    example!(v, check_moment!(2013, 2, 12, 4, 30, 0), "now", "right now", "just now");
    example!(v, check_moment!(2013, 2, 12), "today", "at this time");
    example!(v, check_moment!(2013, 2, 11), "yesterday");
    example!(v, check_moment!(2013, 2, 13), "tomorrow");
    example!(v, check_moment!(2013, 2, 18), "monday", "mon.", "this monday"); // {"day-of-week": 1}
    example!(v, check_moment!(2013, 2, 18), "Monday, Feb 18", "Mon, February 18"); // {"day-of-week": 1, "day": 18, "month": 2}
    example!(v, check_moment!(2013, 2, 19), "tuesday", "Tuesday the 19th", "Tuesday 19th");
    example!(v, check_moment!(2013, 2, 14), "thursday", "thu", "thu.");
    example!(v, check_moment!(2013, 2, 15), "friday", "fri", "fri.");
    example!(v, check_moment!(2013, 2, 16), "saturday", "sat", "sat.");
    example!(v, check_moment!(2013, 2, 17), "sunday", "sun", "sun.");
    example!(v, check_moment!(2013, 3, 1), "the 1st of march", "first of march", "march first"); //{"day": 1, "month": 3}
    example!(v, check_moment!(2013, 3, 3), "march 3");
    example!(v, check_moment!(2013, 3, 15), "the ides of march"); // {"month": 3}
    example!(v, check_moment!(2015, 3, 3), "march 3 2015", "march 3rd 2015", "march third 2015", "3/3/2015", "3/3/15", "2015-3-3", "2015-03-03"); //{"day": 3, "month": 3, "year": 2015}
    example!(v, check_moment!(2013, 2, 15), "on the 15", "on the 15th"); //{"day": 15}
    example!(v, check_moment!(2013, 2, 15), "the 15th of february", "15 of february", "february the 15th", "february 15", "15th february", "2/15", "on 2/15", "February 15"); // {"day": 15, "month": 2}
    example!(v, check_moment!(2013, 8, 8), "Aug 8"); //{"day": 8, "month": 8}
    example!(v, check_moment!(2014, 10), "October 2014"); //{"year": 2014, "month": 10}
    example!(v, check_moment!(1974, 10, 31), "10/31/1974", "10/31/74", "10-31-74"); // {"day": 31, "month": 10, "year": 1974}
    example!(v, check_moment!(2015, 4, 14), "14april 2015", "April 14, 2015", "14th April 15");
    example!(v, check_moment!(2013, 2, 19), "next tuesday");
    example!(v, check_moment!(2013, 2, 22), "friday after next");
    example!(v, check_moment!(2013, 3), "next March");
    example!(v, check_moment!(2014, 3), "March after next");
    example!(v, check_moment!(2013, 2, 10), "Sunday, Feb 10");
    example!(v, check_moment!(2013, 2, 13), "Wed, Feb13");
    example!(v, check_moment!(2013, 2, 18), "Monday, Feb 18", "Mon, February 18");
    example!(v, check_moment!(2013, 2, 11), "this week", "current week", "coming week");
    example!(v, check_moment!(2013, 2, 4), "last week", "past week", "previous week");
    example!(v, check_moment!(2013, 2, 18), "next week", "the following week");
    example!(v, check_moment!(2013, 1), "last month");
    example!(v, check_moment!(2013, 3), "next month");
    example!(v, check_moment!(2013, 1, 1), "this quarter", "this qtr");
    example!(v, check_moment!(2013, 4, 1), "next quarter", "next qtr");
    example!(v, check_moment!(2013, 7, 1), "third quarter", "3rd quarter", "third qtr", "3rd qtr", "the 3rd qtr");
    example!(v, check_moment!(2018, 10, 1), "4th quarter 2018", "4th qtr 2018", "the 4th qtr of 2018");
    example!(v, check_moment!(2012), "last year", "last yr");
    example!(v, check_moment!(2013), "this year", "current year", "this yr");
    example!(v, check_moment!(2014), "next year", "next yr");
    example!(v, check_moment!(2013, 2, 10), "last sunday", "sunday from last week", "last week's sunday");
    example!(v, check_moment!(2013, 2, 5), "last tuesday");
    example!(v, check_moment!(2013, 2, 13), "next wednesday");
    example!(v, check_moment!(2013, 2, 20), "wednesday of next week", "wednesday next week", "wednesday after next");
    example!(v, check_moment!(2013, 2, 22), "friday after next");
    example!(v, check_moment!(2013, 2, 11), "monday of this week");
    example!(v, check_moment!(2013, 2, 12), "tuesday of this week");
    example!(v, check_moment!(2013, 2, 13), "wednesday of this week");
    example!(v, check_moment!(2013, 2, 14), "the day after tomorrow");
    example!(v, check_moment!(2013, 2, 14, 17), "day after tomorrow 5pm");
    example!(v, check_moment!(2013, 2, 10), "the day before yesterday");
    example!(v, check_moment!(2013, 2, 10, 8), "day before yesterday 8am");
    example!(v, check_moment!(2013, 3, 25), "last Monday of March");
    example!(v, check_moment!(2014, 3, 30), "last Sunday of March 2014");
    example!(v, check_moment!(2013, 10, 3), "third day of october");
    example!(v, check_moment!(2014, 10, 6), "first week of october 2014");
    example!(v, check_moment!(2013, 10, 7), "the week of october 6th", "the week of october 7th");
    example!(v, check_moment!(2015, 10, 31), "last day of october 2015", "last day in october 2015");
    example!(v, check_moment!(2014, 9, 22), "last week of september 2014");
    //nth of
    example!(v, check_moment!(2013, 10, 1), "first tuesday of october", "first tuesday in october");
    example!(v, check_moment!(2014, 9, 16), "third tuesday of september 2014");
    example!(v, check_moment!(2014, 10, 1), "first wednesday of october 2014");
    example!(v, check_moment!(2014, 10, 8), "second wednesday of october 2014");
    example!(v, check_moment!(2015, 1, 13), "third tuesday after christmas 2014");
    example!(v, check_moment!(2013, 2, 13, 3), "at 3am", "3 in the AM", "at 3 AM", "3 oclock am", "at three am");
    example!(v, check_moment!(2013, 2, 12, 3, 18), "3:18am", "3:18a");
    example!(v, check_moment!(2013, 2, 12, 15), "at 3pm", "@ 3pm", "3PM", "3pm", "3 oclock pm", "3 o'clock in the afternoon");
    example!(v, check_moment!(2013, 2, 12, 15), "3ish pm", "3pm approximately", "at about 3pm");
    example!(v, check_moment!(2013, 2, 12, 15, 15), "at 15 past 3pm", "a quarter past 3pm", "3:15 in the afternon", "15:15", "3:15pm", "3:15PM", "3:15p");
    example!(v, check_moment!(2013, 2, 12, 15, 20), "at 20 past 3pm", "3:20 in the afternoon", "3:20 in afternoon", "twenty after 3pm", "3:20p");
    example!(v, check_moment!(2013, 2, 12, 15, 30), "at half past three pm", "half past 3 pm", "15:30", "3:30pm", "3:30PM", "330 p.m.", "3:30 p m");
    example!(v, check_moment!(2013, 2, 12, 15, 30), "3:30", "half three");
    example!(v, check_moment!(2013, 2, 12, 15, 23, 24), "15:23:24");
    example!(v, check_moment!(2013, 2, 12, 11, 45), "a quarter to noon", "11:45am", "15 to noon"); // Ambiguous with interval
    example!(v, check_moment!(2013, 2, 12, 20), "8 tonight", "eight tonight", "8 this evening");
    //Mixing date and time
    example!(v, check_moment!(2013, 9, 20, 19, 30), "at 7:30 PM on Fri, Sep 20");
    example!(v, check_moment!(2013, 2, 16, 9), "at 9am on Saturday", "on Saturday for 9am");
    example!(v, check_moment!(2014, 7, 18, 19, 0), "Fri, Jul 18, 2014 07:00 PM");
    example!(v, check_moment!(2013, 2, 12, 4, 30, 1), "in a sec", "one second from now");
    example!(v, check_moment!(2013, 2, 12, 4, 31, 0), "in a minute", "in one minute");
    example!(v, check_moment!(2013, 2, 12, 4, 32, 0), "in 2 minutes", "in 2 more minutes", "2 minutes from now");
    example!(v, check_moment!(2013, 2, 12, 5, 30, 0), "in 60 minutes");
    example!(v, check_moment!(2013, 2, 12, 4, 45, 0), "about a quarter of an hour", "about 1/4h", "about 1/4 h", "about 1/4 hour");
    example!(v, check_moment!(2013, 2, 12, 5, 0, 0), "in half an hour", "in 1/2h", "in 1/2 h", "in 1/2 hour");
    example!(v, check_moment!(2013, 2, 12, 5, 15, 0), "for three-quarters of an hour", "for 3/4h", "for 3/4 h", "for 3/4 hour");
    example!(v, check_moment!(2013, 2, 12, 7, 0, 0), "in 2.5 hours", "in 2 and an half hours");
    example!(v, check_moment!(2013, 2, 12, 5, 30), "in one hour", "in 1h");
    example!(v, check_moment!(2013, 2, 12, 6, 30), "in a couple hours", "in a couple of hours");
    example!(v, check_moment!(2013, 2, 12, 7, 30), "in a few hours", "in few hours");
    example!(v, check_moment!(2013, 2, 13, 4, 30), "in 24 hours", "in 24hrs", "in 24 hrs");
    example!(v, check_moment!(2013, 2, 13, 4), "in a day", "a day from now");
    example!(v, check_moment!(2016, 2), "3 years from today");
    example!(v, check_moment!(2013, 2, 19, 4), "in 7 days");
    example!(v, check_moment!(2013, 2, 19), "in 1 week", "in a week");
    example!(v, check_moment!(2013, 2, 5, 4), "7 days ago");
    example!(v, check_moment!(2013, 1, 29, 4), "14 days ago", "a fortnight ago");
    example!(v, check_moment!(2013, 2, 5), "a week ago", "one week ago", "1 week ago");
    example!(v, check_moment!(2013, 1, 22), "three weeks ago");
    example!(v, check_moment!(2012, 11, 12), "three months ago");
    example!(v, check_moment!(2011, 2), "two years ago");
    example!(v, check_moment!(1954), "1954");
    example!(v, check_moment!(2013, 2, 19, 4), "7 days hence");
    example!(v, check_moment!(2013, 2, 26, 4), "14 days hence", "a fortnight hence");
    example!(v, check_moment!(2013, 2, 19), "a week hence", "one week hence", "1 week hence");
    example!(v, check_moment!(2013, 3, 5), "three weeks hence");
    example!(v, check_moment!(2013, 5, 12), "three months hence");
    example!(v, check_moment!(2015, 2), "two years hence");
    example!(v, check_moment!(2013, 12), "one year after christmas");
    example!(v, check_moment_span!([2013, 6, 21], [2013, 9, 24]), "this summer", "current summer");
    example!(v, check_moment_span!([2012, 12, 21], [2013, 3, 21]), "this winter");
    example!(v, check_moment!(2013, 12, 25), "xmas", "christmas", "christmas day");
    example!(v, check_moment!(2013, 12, 31), "new year's eve", "new years eve");
    example!(v, check_moment!(2014, 1, 1), "new year's day", "new years day");
    example!(v, check_moment!(2013, 2, 14), "valentine's day", "valentine day");
    example!(v, check_moment!(2013, 5, 27), "memorial day");
    example!(v, check_moment!(2013, 5, 12), "Mother's Day");
    example!(v, check_moment!(2013, 6, 16), "Father's Day");
    example!(v, check_moment_span!([2013, 5, 24, 18], [2013, 5, 28, 0]), "memorial day week-end");
    example!(v, check_moment!(2013, 7, 4), "independence day", "4th of July", "4 of july");
    example!(v, check_moment!(2013, 9, 2), "labor day");
    example!(v, check_moment_span!([2013, 8, 30, 18], [2013, 9, 3, 0]), "labor day weekend");
    example!(v, check_moment!(2013, 10, 31), "halloween");
    example!(v, check_moment!(2013, 11, 28), "thanksgiving day", "thanksgiving");
    example!(v, check_moment_span!([2013, 2, 12, 18], [2013, 2, 13, 00]), "this evening", "today evening", "tonight");
    example!(v, check_moment_span!([2013, 2, 8, 18], [2013, 2, 11, 00]), "this past weekend");
    example!(v, check_moment_span!([2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "last 2 seconds", "last two seconds");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "next 3 seconds", "next three seconds");
    example!(v, check_moment_span!([2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "last 2 minutes", "last two minutes");
    example!(v, check_moment_span!([2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "next 3 minutes", "next three minutes");
    example!(v, check_moment_span!([2013, 2, 12, 3], [2013, 2, 12, 4]), "last 1 hour", "last 1 hr", "last one hour");
    example!(v, check_moment_span!([2013, 2, 11, 4], [2013, 2, 12, 4]), "last 24 hours", "last twenty four hours", "last twenty four hrs", "last 24 hrs", "last 24hrs");
    example!(v, check_moment_span!([2013, 2, 12, 5], [2013, 2, 12, 8]), "next 3 hours", "next three hours");
    example!(v, check_moment_span!([2013, 2, 10], [2013, 2, 12]), "last 2 days", "last two days", "past 2 days");
    example!(v, check_moment_span!([2013, 2, 13], [2013, 2, 16]), "next 3 days", "next three days");
    example!(v, check_moment_span!([2013, 2, 13], [2013, 2, 16]), "next few days");
    example!(v, check_moment_span!([2013, 1], [2013, 2]), "last 2 weeks", "last two weeks", "past 2 weeks");
    example!(v, check_moment_span!([2013, 2], [2013, 3]), "next 3 weeks", "next three weeks");
    example!(v, check_moment_span!([2012, 12], [2013, 02]), "last 2 months", "last two months");
    example!(v, check_moment_span!([2013, 3], [2013, 6]), "next 3 months", "next three months");
    example!(v, check_moment_span!([2011], [2013]), "last 2 years", "last two years");
    example!(v, check_moment_span!([2014], [2017]), "next 3 years", "next three years");
    example!(v, check_moment_span!([2013, 7, 13], [2013, 7, 16]), "July 13-15", "July 13 to 15", "July 13 thru 15", "July 13 through 15", "July 13 - July 15");
    example!(v, check_moment_span!([2013, 8, 8], [2013, 8, 13]), "Aug 8 - Aug 12");
    example!(v, check_moment_span!([2013, 2, 12, 9, 30], [2013, 2, 12, 11, 1]), "9:30 - 11:00");
    example!(v, check_moment_span!([2013, 2, 14, 9, 30], [2013, 2, 14, 11, 1]), "from 9:30 - 11:00 on Thursday", "between 9:30 and 11:00 on thursday", "9:30 - 11:00 on Thursday", "later than 9:30 but before 11:00 on Thursday", "Thursday from 9:30 to 11:00","from 9:30 untill 11:00 on thursday", "Thursday from 9:30 untill 11:00", "9:30 till 11:00 on Thursday");
    example!(v, check_moment_span!([2013, 2, 14, 9], [2013, 2, 14, 12]), "Thursday from 9a to 11a");
    example!(v, check_moment_span!([2013, 2, 12, 11, 30], [2013, 2, 12, 13, 31]), "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30");
    example!(v, check_moment!(2013, 9, 21, 13, 30), "1:30 PM on Sat, Sep 21");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "within 2 weeks");
    example!(v, check_moment!(2013, 2, 12, 14, 0), "until 2:00pm", "through 2:00pm");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 0], [2013, 2, 12, 14]), "by 2:00pm");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 0], [2013, 2, 13, 0]), "by EOD");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 0], [2013, 3, 1, 0]), "by EOM");
    example!(v, check_moment_span!([2013, 2, 12, 4, 30, 0], [2013, 4, 1, 0]), "by the end of next month");
    example!(v, check_moment!(2013, 2, 12, 14), "today at 2pm", "at 2pm");
    example!(v, check_moment!(2013, 4, 25, 16, 0), "4/25 at 4:00pm");
    example!(v, check_moment!(2013, 2, 13, 15), "3pm tomorrow");
    example!(v, check_moment!(2013, 2, 12, 14), "after 2 pm");
    example!(v, check_moment!(2013, 2, 17, 4), "after 5 days");
    example!(v, check_moment!(2013, 2, 12, 11), "before 11 am");
    example!(v, check_moment_span!([2013, 2, 12, 12], [2013, 2, 12, 19]), "in the afternoon");
    example!(v, check_moment!(2013, 2, 12, 13, 30), "at 1:30pm", "1:30pm");
    example!(v, check_moment!(2013, 2, 12, 4, 45, 0), "in 15 minutes");
    example!(v, check_moment_span!([2013, 2, 12, 13], [2013, 2, 12, 17]), "after lunch");
    example!(v, check_moment!(2013, 2, 12, 10, 30), "10:30");
    example!(v, check_moment_span!([2013, 2, 12, 4], [2013, 2, 12, 12]), "morning"); //how should we deal with fb morning
    example!(v, check_moment!(2013, 2, 18), "next monday");
    example!(v, check_moment!(2013, 2, 12, 12), "at 12pm", "at noon");
    example!(v, check_moment!(2013, 2, 13, 0), "at 12am", "at midnight");
    example!(v, check_moment!(2013, 3), "March", "in March");
    example!(v, check_moment!(2016, 12, 15), "12.15.2016", "12.15.16");
    v
}

pub fn examples_numbers() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
    example!(v, check_integer(0), "0", "naught", "nought", "zero", "nil");
    example!(v, check_integer(1), "1", "one", "single");
    example!(v, check_integer(2), "2", "two", "a pair");
    example!(v, check_integer(33), "33", "thirty three", "0033");
    example!(v, check_integer(14), "14", "fourteen");
    example!(v, check_integer(16), "16", "sixteen");
    example!(v, check_integer(17), "17", "seventeen");
    example!(v, check_integer(18), "18", "eighteen");
    example!(v, check_float(1.1), "1.1", "1.10", "01.10");
    example!(v, check_float(0.77), "0.77", ".77");
    example!(v,
             check_integer(100000),
             "100,000",
             "100000",
             "100K",
             "100k");
    example!(v,
             check_integer(3000000),
             "3M",
             "3000K",
             "3000000",
             "3,000,000");
    example!(v,
             check_integer(1200000),
             "1,200,000",
             "1200000",
             "1.2M",
             "1200K",
             ".0012G");
    example!(v,
             check_integer(-1200000),
             "- 1,200,000",
             "-1200000",
             "minus 1,200,000",
             "negative 1200000",
             "-1.2M",
             "-1200K",
             "-.0012G");
    example!(v, check_integer(5000), "5 thousand", "five thousand");
    example!(v, check_integer(122), "one twenty two");
    example!(v, check_integer(200000), "two hundred thousand");
    example!(v, check_integer(21011), "twenty-one thousand eleven");
    example!(v,
             check_integer(721012),
             "seven hundred twenty-one thousand twelve",
             "seven hundred twenty-one thousand and twelve");
    example!(v,
             check_integer(31256721),
             "thirty-one million two hundred fifty-six thousand seven hundred twenty-one");
    example!(v, check_ordinal(4), "the 4th", "4th", "fourth");
    example!(v, check_ordinal(3), "the 3rd", "3rd", "third");
    example!(v, check_ordinal(2), "the 2nd", "2nd", "second");
    example!(v, check_ordinal(21), "the twenty first");
    v
}