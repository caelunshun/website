-- CREATE FUNCTION update_total_downloads_by_update()
--     RETURNS TRIGGER 
--     LANGUAGE PLPGSQL
--     AS
-- $$
-- BEGIN
--     IF NEW.downloads <> OLD.downloads THEN
--         UPDATE 
--             plugin_version_artifacts
--         SET
--             downloads = downloads + NEW.downloads - OLD.downloads,
--             downloads_recent = downloads_recent + NEW.downloads - OLD.downloads
--         WHERE
--             id = NEW.plugin_version_artifact_id
-- 		;
--         UPDATE
--             plugin_versions
--         SET
--             downloads = downloads + NEW.downloads - OLD.downloads,
--             downloads_recent = downloads_recent + NEW.downloads - OLD.downloads
--         ;
--     END IF;
-- 	RETURN NEW;
-- END;
-- $$;

-- CREATE TRIGGER update_total_downloads_by_update_
--     AFTER UPDATE
--     ON plugin_version_artifact_daily_downloads
--     FOR EACH ROW
--     EXECUTE PROCEDURE update_total_downloads_by_update();




-- CREATE FUNCTION update_total_downloads_by_insert()
--     RETURNS TRIGGER 
--     LANGUAGE PLPGSQL
--     AS
-- $$
-- BEGIN
--     UPDATE 
--         plugin_version_artifact
--     SET
--         downloads = downloads + NEW.downloads,
--         downloads_recent = downloads_recent + NEW.downloads
--     ;
-- 	RETURN NEW;
-- END;
-- $$;

-- CREATE TRIGGER update_total_downloads_by_insert_
--     AFTER INSERT
--     ON plugin_version_artifact_daily_downloads
--     FOR EACH ROW
--     EXECUTE PROCEDURE update_total_downloads_by_insert();




-- CREATE PROCEDURE next_day_daily_downloads()
--     LANGUAGE PLPGSQL
-- AS
-- $$
-- BEGIN
--     UPDATE
--         plugin_version_artifact
--     SET
--         downloads_recent = downloads_recent - (
--             SELECT 
--                 donwloads
--             FROM 
--                 plugin_version_artifact_downloads_daily
--             WHERE 
--                 AGE("date") = '91 days'::interval
--         )
-- 	;
-- END;
-- $$;