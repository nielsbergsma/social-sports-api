create table if not exists clubs
(
	id text not null
		constraint clubs_pkey
			primary key,
	data json not null,
	name text default ((data -> 'name'::text) ->> 'raw'::text) not null
);

alter table clubs owner to postgres;

create table if not exists teams
(
	id text not null
		constraint teams_pkey
			primary key,
	data json not null,
	name text default ((data -> 'name'::text) ->> 'raw'::text) not null
);

alter table teams owner to postgres;

create table if not exists communities
(
	id text not null
		constraint communities_pkey
			primary key,
	data json not null,
	name text default ((data -> 'name'::text) ->> 'raw'::text) not null,
	context_club text,
	context_team text,
	members text[] default extract_community_members(data)
);

alter table communities owner to postgres;

create table if not exists posts
(
	id text not null
		constraint posts_pkey
			primary key,
	data json not null,
	community text default ((data -> 'community'::text) ->> 'raw'::text) not null,
	published timestamp default text_to_timestamp((data ->> 'published'::text)) not null
);

alter table posts owner to postgres;

create table if not exists comments
(
	id text not null
		constraint comments_pkey
			primary key,
	data json not null,
	reply_to text default ((data -> 'reply_to'::text) ->> 'raw'::text) not null,
	published timestamp default text_to_timestamp((data ->> 'published'::text)) not null
);

alter table comments owner to postgres;

create table if not exists events
(
	id serial not null
		constraint events_pkey
			primary key,
	kind text not null,
	data json not null,
	time timestamp default now() not null
);

alter table events owner to postgres;

create table if not exists post_reactions
(
	post text not null,
	author text not null,
	emotion text not null,
	data json not null,
	constraint post_reactions_pkey
		primary key (post, author)
);

alter table post_reactions owner to postgres;

create view post_reactions_stats(post, reactions_love, reactions_funny, reactions_celebrate, reactions_support, reactions_insightful) as
	SELECT reaction.post,
       count(love.*)       AS reactions_love,
       count(funny.*)      AS reactions_funny,
       count(celebrate.*)  AS reactions_celebrate,
       count(support.*)    AS reactions_support,
       count(insightful.*) AS reactions_insightful
FROM post_reactions reaction
         LEFT JOIN post_reactions love ON reaction.post = love.post AND love.emotion = 'love'::text
         LEFT JOIN post_reactions funny ON reaction.post = funny.post AND funny.emotion = 'funny'::text
         LEFT JOIN post_reactions celebrate ON reaction.post = celebrate.post AND celebrate.emotion = 'celebrate'::text
         LEFT JOIN post_reactions support ON reaction.post = support.post AND support.emotion = 'support'::text
         LEFT JOIN post_reactions insightful ON reaction.post = insightful.post AND insightful.emotion = 'insightful'::text
GROUP BY reaction.post;

alter table post_reactions_stats owner to postgres;

create function bigint_max() returns bigint
	immutable
	strict
	language sql
as $$
select 9223372036854775807
$$;

alter function bigint_max() owner to postgres;

create function text_to_timestamp(text) returns timestamp without time zone
	immutable
	strict
	language sql
as $$
select $1::timestamp
$$;

alter function text_to_timestamp(text) owner to postgres;

create function extract_community_members(json) returns text[]
	immutable
	strict
	language sql
as $$
select array(select json_array_elements($1->'members')->>'raw')
$$;

alter function extract_community_members(json) owner to postgres;

