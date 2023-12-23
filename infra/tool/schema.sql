CREATE TABLE `blog`
(
    `blog_url`   varchar(255) NOT NULL,
    `posted_at`  datetime    NOT NULL,
    `blog_type`  ENUM ('HATENA', 'AMEBA', 'QIITA', 'ZENN', 'SIZU') NOT NULL,
    `blog_title` varchar(511) NOT NULL,
    `created_at` datetime    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` datetime    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`blog_url`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8;


-- insert into blog.blog (blog_url, posted_at, blog_type, blog_title) select url as blog_url, date as posted_at, type as blog_type, title as blog_title from ningenme.blog;

