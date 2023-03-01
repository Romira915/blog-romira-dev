use std::{rc::Rc, time::Duration};

use gloo::timers::future::sleep;
use yew::{hook, suspense::Suspension, Reducible};

pub(crate) const ARTICLE_JSON: &'static str = r###"
{
  "skip": 0,
  "limit": 100,
  "total": 4,
  "items": [
    {
      "_id": "63fdf9f3a6b1cce86529ab03",
      "_sys": {
        "raw": {
          "createdAt": "2023-02-28T12:56:19.491Z",
          "updatedAt": "2023-02-28T12:59:46.459Z",
          "firstPublishedAt": "2023-02-28T12:59:46.459Z",
          "publishedAt": "2023-02-28T12:59:46.459Z"
        },
        "customOrder": 4,
        "createdAt": "2023-02-28T12:59:46.459Z",
        "updatedAt": "2023-02-28T12:59:46.459Z"
      },
      "title": "Rustでblogサイトを作った",
      "slug": "これはなに",
      "meta": {
        "title": "Rustでblogサイトを作った",
        "description": "Rustでblogサイトを作った",
        "ogImage": {
          "_id": "63fdf864a6b1cce86528b002",
          "altText": "",
          "description": "",
          "fileName": "prtimes-hackathon2023.drawio.png",
          "fileSize": 23345,
          "fileType": "image/png",
          "height": 209,
          "metadata": {},
          "src": "https://blog-romira.imgix.net/df1b4b60-b983-4126-8f63-8a0cddf85254/prtimes-hackathon2023.drawio.png",
          "title": "",
          "width": 200
        }
      },
      "body": "<h1>はじめに</h1>\n<p>これはRustでブログサイトを構築した話</p>\n<h1>構成</h1>\n<p>これはこうであれはあれしたRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作ったRustでblogサイトを作った</p>\n<p><img src=\"https://blog-romira.imgix.net/df1b4b60-b983-4126-8f63-8a0cddf85254/prtimes-hackathon2023.drawio.png\" alt=\"prtimes-hackathon2023.drawio.png\"></p>\n<h2>Rust Code</h2>\n<pre><code class=\"language-Rust\">use yew::{function_component, html, Html, Properties};\n\n#[derive(PartialEq, Properties)]\npub struct HeadlineProps {}\n\n#[function_component]\npub fn Headline(props: &amp;HeadlineProps) -&gt; Html {\n    let HeadlineProps {} = props;\n    html! {\n        &lt;div&gt;&lt;/div&gt;\n    }\n}\n\n</code></pre>\n<h2>表</h2>\n<table>\n<thead>\n<tr>\n<th>Head</th>\n<th>Head</th>\n<th>Head</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>Data</td>\n<td>Data</td>\n<td>Data</td>\n</tr>\n<tr>\n<td>Data</td>\n<td>Data</td>\n<td>Data</td>\n</tr>\n</tbody>\n</table>\n<h2>Code</h2>\n<p><code>This is Code</code></p>\n<h2></h2>\n<blockquote>\n<p>testest</p>\n</blockquote>\n<h1>最後に</h1>\n<p>これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。これはテスト投稿。</p>\n",
      "coverImage": {
        "_id": "63fdf864a6b1cce86528b002",
        "altText": "",
        "description": "",
        "fileName": "prtimes-hackathon2023.drawio.png",
        "fileSize": 23345,
        "fileType": "image/png",
        "height": 209,
        "metadata": {},
        "src": "https://blog-romira.imgix.net/df1b4b60-b983-4126-8f63-8a0cddf85254/prtimes-hackathon2023.drawio.png",
        "title": "",
        "width": 200
      },
      "author": {
        "_id": "63fdfa40a6b1cce86529ebcf",
        "_sys": {
          "raw": {
            "createdAt": "2023-02-28T12:57:36.944Z",
            "updatedAt": "2023-02-28T12:57:36.944Z",
            "firstPublishedAt": "2023-02-28T12:57:36.944Z",
            "publishedAt": "2023-02-28T12:57:36.944Z"
          },
          "customOrder": 2,
          "createdAt": "2023-02-28T12:57:36.944Z",
          "updatedAt": "2023-02-28T12:57:36.944Z"
        },
        "fullName": "Romira",
        "profileImage": "63fdfa13a6b1cce86529b2b2",
        "biography": "<p>I'm a Rustacean</p>"
      },
      "categories": [
        {
          "_id": "61b1677c32470a0018d79abf",
          "_sys": {
            "raw": {
              "createdAt": "2021-12-09T02:18:36.889Z",
              "updatedAt": "2023-02-23T10:15:41.662Z",
              "firstPublishedAt": "2021-12-09T02:18:36.889Z",
              "publishedAt": "2021-12-09T02:18:36.889Z"
            },
            "createdAt": "2021-12-09T02:18:36.889Z",
            "updatedAt": "2021-12-09T02:18:36.889Z"
          },
          "name": "Developers",
          "slug": "developers"
        }
      ]
    },
    {
      "_id": "61b1688832470a0018d79f4a",
      "_sys": {
        "raw": {
          "createdAt": "2021-12-09T02:23:04.494Z",
          "updatedAt": "2023-02-23T10:15:40.026Z",
          "firstPublishedAt": "2021-12-09T02:23:04.494Z",
          "publishedAt": "2022-02-16T11:14:36.733Z"
        },
        "createdAt": "2021-12-09T02:23:04.494Z",
        "updatedAt": "2022-02-16T11:14:36.733Z"
      },
      "title": "Fictitiousをクイックにスタートする方法",
      "slug": "article-3",
      "meta": {
        "title": "Fictitiousをクイックにスタートする方法 | Fictitious Blog",
        "description": "このガイドでは、Appの作成からコンテンツの作成、APIでの取得まで順を追って紹介します。",
        "ogImage": null
      },
      "body": "<p><img src=\"https://ik.imagekit.io/newt/tr:w-1000,h-1000,c-at_max/618884b9b2f01f00183628e7/377e51be-6a81-4f24-a108-f07c54ee4757/nubelson-fernandes-gTs2w7bu3Qo-unsplash.jpg\" alt=\"nubelson-fernandes-gTs2w7bu3Qo-unsplash.jpg\"></p>\n<h2>ステップ1</h2>\n<p>このガイドでは、Appの作成からコンテンツの作成、APIでの取得まで順を追って紹介します。<br>\nまずはAppを追加しましょう。「Appを追加」から「新規作成」を選択します。<code>App UID</code>はAPIで情報を取得する時のURLとして利用されます。ここで設定する情報は、あとで変更可能です。</p>\n<h2>ステップ2</h2>\n<p>「モデルを作成」を選択し、モデルを新規作成します。モデル UIDはAPIで情報を取得する時のURLとして利用されます。ここで設定する情報は、あとで変更可能です。続いて、モデルのフィールドを追加します。ここではタイトル（<code>フィールドID: title</code>）と本文（<code>フィールドID: body</code>）を設定しました。設定が終わったら、保存ボタンを押してください。</p>\n<p>APIのエンドポイントは以下のようになっているため、まずプロジェクトUID・App UID・モデルUIDを確認します。</p>\n<pre><code>https://{projectUid}.cdn.newt.so/v1/{appUid}/{modelUid}\n</code></pre>\n<h2>最後にFictitiousの特徴を紹介します</h2>\n<ol>\n<li>Appによる分かり易いサイト設計とコンテンツ管理</li>\n<li>コンテンツに応じた管理画面の作成</li>\n<li>コンテンツ管理をもっと楽しく。</li>\n<li>Appテンプレートで簡単スタート</li>\n</ol>\n",
      "coverImage": null,
      "author": {
        "_id": "61a7359d8131ac001847fbb4",
        "_sys": {
          "raw": {
            "createdAt": "2021-12-01T08:43:09.433Z",
            "updatedAt": "2023-02-23T10:15:41.248Z",
            "firstPublishedAt": "2021-12-01T08:43:09.433Z",
            "publishedAt": "2022-02-17T01:39:11.903Z"
          },
          "createdAt": "2021-12-01T08:43:09.433Z",
          "updatedAt": "2022-02-17T01:39:11.903Z"
        },
        "fullName": "Donna Thomason",
        "profileImage": null,
        "biography": "<p>2021年にFictitiousを創業。前職のPetlassianではマーケティング責任者としてグローバル・マーケティングを担当し、オンライン販売モデルの構築・拡大に貢献</p>"
      },
      "categories": [
        {
          "_id": "61b1677c32470a0018d79abf",
          "_sys": {
            "raw": {
              "createdAt": "2021-12-09T02:18:36.889Z",
              "updatedAt": "2023-02-23T10:15:41.662Z",
              "firstPublishedAt": "2021-12-09T02:18:36.889Z",
              "publishedAt": "2021-12-09T02:18:36.889Z"
            },
            "createdAt": "2021-12-09T02:18:36.889Z",
            "updatedAt": "2021-12-09T02:18:36.889Z"
          },
          "name": "Developers",
          "slug": "developers"
        }
      ]
    },
    {
      "_id": "61b1686732470a0018d79e97",
      "_sys": {
        "raw": {
          "createdAt": "2021-12-09T02:22:31.979Z",
          "updatedAt": "2023-02-23T10:15:41.091Z",
          "firstPublishedAt": "2021-12-09T02:22:31.979Z",
          "publishedAt": "2022-02-16T11:31:15.888Z"
        },
        "createdAt": "2021-12-09T02:22:31.979Z",
        "updatedAt": "2022-02-16T11:31:15.888Z"
      },
      "title": "Fictitious Communityがパンデミックの間イベントをどのように主催したか",
      "slug": "article-2",
      "meta": {
        "title": "Fictitious Communityがパンデミックの間イベントをどのように主催したか",
        "description": "コミュニティは常にFictitiousの中心でした。多くの人々がコミュニティに加わるにつれて、世界中のユーザーが独自のコミュニティを形成し始めまていました。",
        "ogImage": null
      },
      "body": "<p><img src=\"https://ik.imagekit.io/newt/tr:w-1000,h-1000,c-at_max/618884b9b2f01f00183628e7/d8c6807f-9870-4da2-b0a3-f86fefb14df8/dylan-gillis-KdeqA3aTnBY-unsplash.jpg\" alt=\"dylan-gillis-KdeqA3aTnBY-unsplash.jpg\"></p>\n<h2>Fictitiousコミュニティ</h2>\n<p>コミュニティは常にFictitiousの中心でした。多くの人々がコミュニティに加わるにつれて、世界中のユーザーが独自のコミュニティを形成し始めまていました。<br>\nしかし、世界的なパンデミックがおき行動は制限されました。そんな中でもコミュニティは、世界中で素晴らしいイベントを何度も開催してくれました。</p>\n<ul>\n<li>4/4 Fictitious Camp</li>\n<li>6/9 Fictitious conference 2021</li>\n<li>9/5 Meetup! Fictitious</li>\n</ul>\n<p><strong>オンラインイベントの大きなチャンスは、より多くのオーディエンスにリーチできること</strong>。コミュニティは以前にもまして国際的になり、予想外の成長を遂げました。</p>\n<h2>Thank you</h2>\n<p>今年イベントに参加してくださった皆様、ありがとうございました。来年もこの取り組みを継続するために取り組んでいきます。今後のイベントにも是非参加してください。リスト</p>\n",
      "coverImage": null,
      "author": {
        "_id": "61a7359d8131ac001847fbb4",
        "_sys": {
          "raw": {
            "createdAt": "2021-12-01T08:43:09.433Z",
            "updatedAt": "2023-02-23T10:15:41.248Z",
            "firstPublishedAt": "2021-12-01T08:43:09.433Z",
            "publishedAt": "2022-02-17T01:39:11.903Z"
          },
          "createdAt": "2021-12-01T08:43:09.433Z",
          "updatedAt": "2022-02-17T01:39:11.903Z"
        },
        "fullName": "Donna Thomason",
        "profileImage": null,
        "biography": "<p>2021年にFictitiousを創業。前職のPetlassianではマーケティング責任者としてグローバル・マーケティングを担当し、オンライン販売モデルの構築・拡大に貢献</p>"
      },
      "categories": [
        {
          "_id": "61a73adc6c236700180c0da8",
          "_sys": {
            "raw": {
              "createdAt": "2021-12-01T09:05:32.437Z",
              "updatedAt": "2023-02-23T10:15:41.989Z",
              "firstPublishedAt": "2021-12-01T09:05:32.437Z",
              "publishedAt": "2021-12-09T02:18:29.098Z"
            },
            "createdAt": "2021-12-01T09:05:32.437Z",
            "updatedAt": "2021-12-09T02:18:29.098Z"
          },
          "name": "Community",
          "slug": "community"
        }
      ]
    },
    {
      "_id": "61b1682a32470a0018d79d59",
      "_sys": {
        "raw": {
          "createdAt": "2021-12-09T02:21:30.173Z",
          "updatedAt": "2023-02-23T10:15:41.158Z",
          "firstPublishedAt": "2021-12-09T02:21:30.173Z",
          "publishedAt": "2021-12-09T02:21:30.173Z"
        },
        "createdAt": "2021-12-09T02:21:30.173Z",
        "updatedAt": "2021-12-09T02:21:30.173Z"
      },
      "title": "Fictitious Communities Summit 2021のレポート",
      "slug": "article-1",
      "body": "<p><img src=\"https://ik.imagekit.io/newt/tr:w-1000,h-1000,c-at_max/618884b9b2f01f00183628e7/a359dd8b-d928-447b-8792-c5cbbbb4aaee/kylie-lugo-t0BavJY0M-U-unsplash.jpg\" alt=\"kylie-lugo-t0BavJY0M-U-unsplash.jpg\"></p>\n<h2>Fictitiousの新機能の発表</h2>\n<p>ファウンダー兼CEOのDonna Thomasonから、Fictitiousの新機能を発表しました。また、Fictitiousをモバイルでも使える新しいツールも発表しました。<a href=\"#\">詳しくは、こちらをご覧ください。</a></p>\n<h3>アワード受賞者</h3>\n<p>毎年Fictitious Community Acceleratorプログラムへの応募者から、すばらしいコミュニティビルダーを表彰しています。今年の受賞者はJohn Smith。Johnはパンデミックの最中にコミュニティを始め、1,000人以上の人々を集めて親切な話を共有しました。</p>\n",
      "coverImage": null,
      "author": {
        "_id": "61a7359d8131ac001847fbb4",
        "_sys": {
          "raw": {
            "createdAt": "2021-12-01T08:43:09.433Z",
            "updatedAt": "2023-02-23T10:15:41.248Z",
            "firstPublishedAt": "2021-12-01T08:43:09.433Z",
            "publishedAt": "2022-02-17T01:39:11.903Z"
          },
          "createdAt": "2021-12-01T08:43:09.433Z",
          "updatedAt": "2022-02-17T01:39:11.903Z"
        },
        "fullName": "Donna Thomason",
        "profileImage": null,
        "biography": "<p>2021年にFictitiousを創業。前職のPetlassianではマーケティング責任者としてグローバル・マーケティングを担当し、オンライン販売モデルの構築・拡大に貢献</p>"
      },
      "categories": [
        {
          "_id": "61a73adc6c236700180c0da8",
          "_sys": {
            "raw": {
              "createdAt": "2021-12-01T09:05:32.437Z",
              "updatedAt": "2023-02-23T10:15:41.989Z",
              "firstPublishedAt": "2021-12-01T09:05:32.437Z",
              "publishedAt": "2021-12-09T02:18:29.098Z"
            },
            "createdAt": "2021-12-01T09:05:32.437Z",
            "updatedAt": "2021-12-09T02:18:29.098Z"
          },
          "name": "Community",
          "slug": "community"
        }
      ]
    }
  ]
}
"###;

#[derive(PartialEq)]
pub struct SleepState {
    s: Suspension,
}

impl SleepState {
    fn new() -> Self {
        let s = Suspension::from_future(async {
            sleep(Duration::from_secs(5)).await;
        });

        Self { s }
    }
}

impl Reducible for SleepState {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        Self::new().into()
    }
}

#[hook]
pub fn use_sleep() -> yew::suspense::SuspensionResult<Rc<dyn Fn()>> {
    let sleep_state = yew::use_reducer(SleepState::new);

    if sleep_state.s.resumed() {
        Ok(Rc::new(move || sleep_state.dispatch(())))
    } else {
        Err(sleep_state.s.clone())
    }
}
