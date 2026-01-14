package browser

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strings"

	"github.com/yashodhanketkar/louarch/src/fs"
	"github.com/yashodhanketkar/louarch/src/utils"
)

func cleanupBookmarks(bookmarks []string) []string {
	if len(bookmarks) > 0 && bookmarks[len(bookmarks)-1] == "" {
		bookmarks = slices.Delete(bookmarks, len(bookmarks)-1, len(bookmarks))
	}
	bookmarks = append(bookmarks, "Add", "Remove")
	return bookmarks
}

func getBookmarks() []string {
	var bookmarks = make([]string, 0)
	bookmarksRaw, err := os.ReadFile(fs.AppConfig.BookmarksFile)

	if err != nil {
		fs.CreateFile(fs.AppConfig.BookmarksFile, "Creating bookmarks file")
	} else {
		bookmarks = strings.Split(string(bookmarksRaw), "\n")
	}

	bookmarks = cleanupBookmarks(bookmarks)

	return bookmarks
}

func openBookmark(bookmark string) {
	utils.CmdRunner(fs.AppConfig.Browser, bookmark)
}

func selectOptions() {
	bookmarks := getBookmarks()
	selected, ok := utils.WofiPrompt("Select bookmark ", bookmarks...)
	if !ok {
		log.Fatalf("Error occured while selecting bookmark")
	}

	switch selected {
	case "Add":
		addBookmark()
	case "Remove":
		removeBookmark()
	default:
		openBookmark(selected)
	}
}

func removeBookmark() {
	bookmarks := getBookmarks()

	bookmarks = slices.DeleteFunc(bookmarks, func(s string) bool {
		return s == "Add" || s == "Remove"
	})

	bookmark, ok := utils.WofiPrompt("Select bookmark to remove", bookmarks...)
	if !ok {
		log.Fatalf("Error occured while selecting bookmark")
	}

	file, err := os.Open(fs.AppConfig.BookmarksFile)
	if err != nil {
		log.Fatalf("Error opening bookmarks file %s\n%v", fs.AppConfig.BookmarksFile, err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	if scanner.Err() != nil {
		log.Fatalf("Error reading bookmarks file\n%v", err)
	}

	for scanner.Scan() {
		if scanner.Text() != bookmark {
			lines = append(lines, scanner.Text())
		}
	}

	tmp := fs.AppConfig.BookmarksFile + ".tmp"
	os.WriteFile(tmp, []byte(strings.Join(lines, "\n")+"\n"), 0644)
	os.Rename(tmp, fs.AppConfig.BookmarksFile)
}

func addBookmark() {
	bookmark, ok := utils.WofiPrompt("Enter bookmark to add")
	if !ok {
		log.Fatalf("Error occured while adding bookmark")
	}

	currentBookmarks := getBookmarks()
	if slices.Contains(currentBookmarks, bookmark) {
		return
	}

	file, err := os.OpenFile(
		fs.AppConfig.BookmarksFile,
		os.O_APPEND|os.O_CREATE|os.O_WRONLY,
		0644,
	)
	if err != nil {
		log.Fatalf("Error opening bookmarks file %s\n%v", fs.AppConfig.BookmarksFile, err)
	}
	defer file.Close()

	if _, err := file.WriteString(bookmark + "\n"); err != nil {
		log.Fatalf("Error writing bookmark to file %s\n%v", fs.AppConfig.BookmarksFile, err)
	}
}
