subshader "Cloudcar_Engine_FX_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.337255 0.337255 0.337255;
	materialSpecularPower 12.5;
	transparent true;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "texture/Effects/Cloudcar_exhaust";
}

