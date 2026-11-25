Class CacheV1.MCP.Run Extends %RegisteredObject
{

/// Exec helper for the cache-v1-mcp MCP server (Atelier API v1, Caché 2016.2.3).
/// Invoke as: CALL CacheV1_MCP.Run_Exec(?) with the ObjectScript code as the parameter.
/// Output written by the code is captured by the Atelier query action and returned in
/// its console array. Installed and kept in sync by mcp/cache-v1.mjs; this class is the
/// repo copy - the server embeds the same source (HELPER_SOURCE) and syncs it on use.
ClassMethod Exec(code As %String = "") As %String [ SqlProc ]
{
	If (code = "") Write "*** EXECUTION FAILED ***: no code provided", ! Quit ""
	Try {
		Set tFlat = ..Flatten(code)
		XECUTE tFlat
	} Catch e {
		Write !, "*** EXECUTION FAILED ***: "_e.AsSystemError(), !
	}
	Quit ""
}

/// Compile documents and report per-name status in a locale-independent form.
/// 
/// The Atelier compile action on 2016.2.3 returns an empty result.content and puts its
/// messages in console as localised prose ("Kompilierung ... erfolgreich abgeschlossen"),
/// so parsing that response is not portable. $System.OBJ.Compile returns a %Status and an
/// error log, which is.
/// 
/// pNames is a comma-separated list of document names; pFlags is a compile qualifier set.
/// A .cls suffix is stripped: $System.OBJ.Compile wants the bare class name and answers
/// "class does not exist" for the document form. Routine extensions (.mac/.int/.inc) are
/// left alone, because those are the names the compiler expects for routines.
ClassMethod Compile(pNames As %String = "", pFlags As %String = "cuk") As %String [ SqlProc ]
{
	Set tCount = $Length(pNames, ",")
	For tI = 1:1:tCount {
		Set tDoc = $Piece(pNames, ",", tI)
		If (tDoc = "") Continue
		Set tName = tDoc
		If ($ZConvert($Extract(tDoc, *-3, *), "L") = ".cls") {
			Set tName = $Extract(tDoc, 1, *-4)
		}
		Kill tErr
		Set tSC = $System.OBJ.Compile(tName, pFlags, .tErr)
		If ($System.Status.IsOK(tSC)) {
			Write "OK ", tDoc, !
		} Else {
			Write "ERROR ", tDoc, ": ", $System.Status.GetErrorText(tSC), !
		}
		Set tJ = ""
		For {
			Set tJ = $Order(tErr(tJ))
			Quit:(tJ = "")
			Write "  ", tErr(tJ), !
		}
	}
	Quit ""
}

/// Collapse multi-line ObjectScript into the single line that XECUTE requires.
/// 
/// XECUTE takes one or more comma-separated arguments, each executed as a line; a newline
/// inside a single argument is a <SYNTAX> error. ObjectScript uses a newline purely as a
/// statement separator, and a space separates commands just as well, so joining the lines
/// with a space is semantically equivalent - apart from two things that would change the
/// meaning and are handled first:
///   * // and ; comments are stripped, or the join would comment out the rest;
///   * blank lines and #; comments are dropped.
/// String literals are tracked so // in "http://x" and ; in "a;b" survive.
ClassMethod Flatten(pCode As %String) As %String
{
	Set tOut = ""
	Set tText = $Translate(pCode, $Char(13), "")
	Set tCount = $Length(tText, $Char(10))
	For tI = 1:1:tCount {
		Set tLine = ..StripComment($Piece(tText, $Char(10), tI))
		Set tBare = $Translate(tLine, " "_$Char(9))
		If (tBare = "") Continue
		If (tBare = "#") Continue
		Set tOut = $Select(tOut = "": tLine, 1: tOut _" "_tLine)
	}
	Quit tOut
}

/// Remove a trailing // or ; comment, ignoring both inside string literals.
/// An escaped quote ("") closes and immediately reopens the literal, so the toggle below
/// stays correct without a special case.
ClassMethod StripComment(pLine As %String) As %String
{
	Set tOut = pLine
	Set tInStr = 0
	Set tDone = 0
	Set tLen = $Length(pLine)
	Set tI = 1
	While ((tI <= tLen) && (tDone = 0)) {
		Set tCh = $Extract(pLine, tI)
		If (tCh = """") {
			Set tInStr = 'tInStr
		} ElseIf (tInStr = 0) {
			If (tCh = ";") {
				Set tOut = $Extract(pLine, 1, tI - 1)
				Set tDone = 1
			} ElseIf (tCh = "/") {
				If ($Extract(pLine, tI + 1) = "/") {
					Set tOut = $Extract(pLine, 1, tI - 1)
					Set tDone = 1
				}
			}
		}
		Set tI = tI + 1
	}
	Quit tOut
}

}
