package;

import Xml;
import sys.db.Sqlite;
import sys.db.Connection;
import sys.io.Process;
import sys.io.File;

using StringTools;

class Main {
	static var DEFAULT_XML = "W:/Volumes/RC3/TEM/VolumeData.xml";

	public static function main() {
		// Use sqlite3 CLI to make a new database
		var dbFile = Date.now().toString().replace(" ", "-").replace(":", "-") + ".db";
		var exitCode = new Process('@echo .save $dbFile | sqlite3').exitCode();
		if (exitCode != 0) {
			trace('failed to create database file $dbFile');
			Sys.exit(exitCode);
		}

		// Open the database for Haxe operations
		var db = Sqlite.open(dbFile);
		db.close();
		trace("hello world");

		// Open the VolumeData.xml for parsing
		var xmlFile = if (Sys.args().length > 0) {
			Sys.args()[0];
		} else DEFAULT_XML;
		// Read the VolumeData.xml's content once and only once, so nothing destructive
		// can happen while migrating:
		var xml = Xml.parse(File.getContent(xmlFile));
		trace(xml);

		// I don't think the root Block element needs to be in the database.

		// TODO stos group links

		// TODO non stos section numbers

		// TODO section links

		// TODO stos maps

		// TODO mappings

		// TODO put the notes file stuff in the database
	}
}
