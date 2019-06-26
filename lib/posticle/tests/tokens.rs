#[macro_use]
extern crate pretty_assertions;
extern crate posticle;

use posticle::tokens::*;
use posticle::Reader;

#[test]
fn extracts_nothing() {
    assert_eq!(
        Reader::from("a string without at signs").into_vec(),
        vec![Token::Text(Text {
            text: "a string without at signs".to_string(),
        })]
    );
}

#[test]
fn extracts_mentions() {
    assert_eq!(
        Reader::from("@mention").into_vec(),
        vec![Token::Mention(Mention {
            username: "mention".to_string(),
            domain:   None,
        })]
    );
    assert_eq!(
        Reader::from("@mention@domain.place").into_vec(),
        vec![Token::Mention(Mention {
            username: "mention".to_string(),
            domain:   Some("domain.place".to_string()),
        })]
    );
    assert_eq!(
        Reader::from("@Mention@Domain.Place").into_vec(),
        vec![Token::Mention(Mention {
            username: "Mention".to_string(),
            domain:   Some("Domain.Place".to_string()),
        })]
    );
}

#[test]
fn extracts_mentions_in_punctuation() {
    assert_eq!(
        Reader::from("(@mention)").into_vec(),
        vec![
            Token::Text(Text {
                text: "(".to_string(),
            }),
            Token::Mention(Mention {
                username: "mention".to_string(),
                domain:   None,
            }),
            Token::Text(Text {
                text: ")".to_string(),
            })
        ]
    );
}

#[test]
fn ignores_invalid_mentions() {
    let mentions = vec![
        "some text @ yo",
        "@@yuser@domain",
        "@xuser@@domain",
        "@@not",
        "@not@",
        "@not@@not",
        "@not@not@not",
    ];

    for mention in mentions {
        assert_eq!(
            Reader::from(mention).into_vec(),
            vec![Token::Text(Text {
                text: mention.to_string(),
            })],
            "ignores_invalid_mentions failed on {}",
            mention
        );
    }
}

#[test]
fn extracts_hashtags() {
    let hashtags = vec!["#hashtag", "#HASHTAG", "#1000followers", "#文字化け"];

    for hashtag in hashtags {
        assert_eq!(
            Reader::from(hashtag).into_vec(),
            vec![Token::Hashtag(Hashtag {
                name: hashtag[1..].to_string(),
            })],
            "extracts_hashtags failed on {}",
            hashtag
        );
    }
}

#[test]
fn extracts_hashtags_in_punctuation() {
    let hashtags = vec!["#hashtag", "#HASHTAG", "#1000followers", "#文字化け"];

    for hashtag in hashtags {
        assert_eq!(
            Reader::from(format!("({})", hashtag)).into_vec(),
            vec![
                Token::Text(Text {
                    text: "(".to_string(),
                }),
                Token::Hashtag(Hashtag {
                    name: hashtag[1..].to_string(),
                }),
                Token::Text(Text {
                    text: ")".to_string(),
                })
            ],
            "extracts_hashtags_in_punctuation failed on {}",
            hashtag
        );
    }
}

#[test]
fn ignores_invalid_hashtags() {
    let hashtags = vec![
        "some text # yo",
        "##not",
        "#not#",
        "#not##not",
        "#not#not#not",
    ];

    for hashtag in hashtags {
        assert_eq!(
            Reader::from(hashtag).into_vec(),
            vec![Token::Text(Text {
                text: hashtag.to_string(),
            })],
            "ignores_invalid_hashtags failed on {}",
            hashtag
        );
    }
}

#[test]
fn extracts_links() {
    let links = vec![
        "http://example.com",
        "http://example.com/path/to/resource?search=foo&lang=en",
        "http://example.com/#!/heck",
        "HTTP://www.ExaMPLE.COM/index.html",
        "http://example.com:8080/",
        "http://test_underscore.example.com",
        "http://☃.net/",
        "http://example.com/Русские_слова",
        "http://example.com/الكلمات_العربية",
        "http://sports.yahoo.com/nfl/news;_ylt=Aom0;ylu=XyZ?slug=ap-superbowlnotebook",
        "http://example.com?foo=$bar.;baz?BAZ&c=d-#top/?stories",
        "http://www.youtube.com/watch?v=g8X0nJHrJ9g&list=PLLLYkE3G1HEAUsnZ-vfTeQ_ZO37DhHhOY-",
        "http://www.example.com/",
    ];

    for link in links {
        assert_eq!(
            Reader::from(link).into_vec(),
            vec![Token::Link(Link {
                url: link.to_string(),
            })],
            "extracts_links failed on {}",
            link
        );
    }
}

#[test]
fn extracts_links_in_punctuation() {
    let links = vec![
        "http://example.com",
        "http://example.com/path/to/resource?search=foo&lang=en",
        "http://example.com/#!/heck",
        "HTTP://www.ExaMPLE.COM/index.html",
        "http://example.com:8080/",
        "http://test_underscore.example.com",
        "http://☃.net/",
        "http://example.com/Русские_слова",
        "http://example.com/الكلمات_العربية",
        "http://sports.yahoo.com/nfl/news;_ylt=Aom0;ylu=XyZ?slug=ap-superbowlnotebook",
        "http://example.com?foo=$bar.;baz?BAZ&c=d-#top/?stories",
        "http://www.youtube.com/watch?v=g8X0nJHrJ9g&list=PLLLYkE3G1HEAUsnZ-vfTeQ_ZO37DhHhOY-",
        "http://www.example.com/",
    ];

    for link in links {
        assert_eq!(
            Reader::from(format!("({})", link)).into_vec(),
            vec![
                Token::Text(Text {
                    text: "(".to_string(),
                }),
                Token::Link(Link {
                    url: link.to_string(),
                }),
                Token::Text(Text {
                    text: ")".to_string(),
                })
            ],
            "extracts_links_in_punctuation failed on {}",
            link
        );
    }
}

#[test]
fn ignores_invalid_links() {
    let links = vec!["x- text http:// yo", "_=_:thing", "nö://thing/else yo"];

    for link in links {
        assert_eq!(
            Reader::from(link).into_vec(),
            vec![Token::Text(Text {
                text: link.to_string(),
            })],
            "ignores_invalid_links failed on {}",
            link
        );
    }
}

#[test]
fn extracts_all() {
    assert_eq!(
        Reader::from("text #hashtag https://example.com @mention text").into_vec(),
        vec![
            Token::Text(Text {
                text: "text ".to_string(),
            }),
            Token::Hashtag(Hashtag {
                name: "hashtag".to_string(),
            }),
            Token::Text(Text {
                text: " ".to_string(),
            }),
            Token::Link(Link {
                url: "https://example.com".to_string(),
            }),
            Token::Text(Text {
                text: " ".to_string(),
            }),
            Token::Mention(Mention {
                username: "mention".to_string(),
                domain:   None,
            }),
            Token::Text(Text {
                text: " text".to_string(),
            }),
        ]
    );
}
